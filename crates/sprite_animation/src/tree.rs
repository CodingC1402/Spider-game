use std::{
    any::type_name,
    collections::{hash_map::Values, HashMap},
    hash::Hash,
};

use crate::prelude::{play_node::PlayNodeResult, AnimNode};

use bevy::{prelude::*, utils::Uuid};

pub struct AnimTreeUpdate {
    pub delay: f32,
    pub index: usize,
}

#[derive(Resource)]
pub struct AnimTree<T>
where
    T: Hash + Eq + Default + ToString,
{
    nodes: HashMap<Uuid, AnimNode<T>>,

    /// Start node is also default nodes for fall back if the next uuid is invalid
    start_node: Uuid,
}

impl<T> AnimTree<T>
where
    T: Hash + Eq + Default + ToString,
{
    pub fn update(
        &self,
        node: &AnimNode<T>,
        delta_time: f32,
        time: f32,
        index: usize,
        state: &T,
    ) -> Result<AnimTreeUpdate, String> {
        match node {
            AnimNode::PlayNode(node) => match node.execute(time, delta_time, index) {
                PlayNodeResult::NextNode(id) => {
                    self.update(self.get_node(id), delta_time, time, index, state)
                }
                PlayNodeResult::Sprite(delay, index) => Ok(AnimTreeUpdate { delay, index }),
            },
            AnimNode::MatchNode(node) => match node.execute(state) {
                Ok(id) => self.update(self.get_node(id), delta_time, time, index, state),
                Err(msg) => Err(format!("Failed to update tree with message: \n{}", msg)),
            },
            AnimNode::ComponentNode(_) => todo!(),
        }
    }

    pub fn get_node(&self, id: Uuid) -> &AnimNode<T> {
        match self.nodes.get(&id) {
            Some(node) => node,
            None => panic!(
                "Can't find node with id {} in tree of state {}",
                id,
                type_name::<T>()
            ),
        }
    }

    pub fn insert_node(&mut self, mut node: AnimNode<T>) -> Result<&mut Self, String> {
        let id = node.get_id();
        id.is_nil().then_some(node.set_default_id());
        self.nodes
            .contains_key(&id)
            .then_some(Err(format!(
                "The id field of node already exist in this tree ({})",
                id.to_string()
            )))
            .unwrap_or_else(|| {
                self.nodes.insert(id, node);
                Ok(self)
            })
    }

    pub fn new(mut start_node: AnimNode<T>) -> Self {
        let mut nodes = HashMap::new();
        start_node.get_id().is_nil().then_some(start_node.set_default_id());
        nodes.insert(start_node.get_id(), start_node);

        let mut instance = Self {
            nodes,
            start_node: Uuid::nil()
        };

        instance.set_default_start_node();
        instance
    }

    pub fn set_default_start_node(&mut self) -> &mut Self {
        self.start_node = self.nodes.values().next().and_then(|x| Some(x.get_id())).unwrap_or(Uuid::nil());
        self
    }

    pub fn set_start_node(&mut self, uuid: &Uuid) -> Result<&mut Self, &str> {
        const NO_UUID_EXIST_ERR: &str = "No uuid exist with this uuid";

        match self.nodes.get(&uuid) {
            Some(node) => {
                self.start_node = node.get_id();
                Ok(self)
            },
            None => Err(NO_UUID_EXIST_ERR),
        }
    }

    pub fn iter(&self) -> Values<'_, Uuid, AnimNode<T>> {
        self.nodes.values().into_iter()
    }
}
