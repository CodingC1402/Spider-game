use std::{
    any::type_name,
    collections::{hash_map::Values, HashMap},
    hash::Hash,
    ops::Deref,
};

use crate::prelude::{play_node::PlayNodeResult, AnimNode};
use bevy::{prelude::*, utils::Uuid};

pub trait AnimTreeWrap<T>: Resource
where
    T: AnimState,
{
    fn get(&self) -> &AnimTree<T>;
}
impl<T> Deref for dyn AnimTreeWrap<T>
where
    T: AnimState,
{
    type Target = AnimTree<T>;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

pub trait AnimState: Hash + Eq + Default + ToString + Send + Sync + 'static {}

pub struct AnimTreeUpdate {
    pub time: f32,
    pub keyframe_index: usize,
    pub atlas_index: usize,
}

#[derive(Resource)]
pub struct AnimTree<T>
where
    T: AnimState,
{
    nodes: HashMap<Uuid, AnimNode<T>>,
    /// Start node is also default nodes for fall back if the next uuid is invalid
    start_node: Uuid,
}

impl<T> AnimTree<T>
where
    T: AnimState,
{
    pub fn update(
        &self,
        delta_time: f32,
        time: f32,
        index: usize,
        state: &T,
    ) -> Result<AnimTreeUpdate, String> {
        self.handle_update(
            self.get_node(self.start_node),
            delta_time,
            time,
            index,
            state,
        )
    }
    fn handle_update(
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
                    self.handle_update(self.get_node(id), delta_time, time, index, state)
                }
                PlayNodeResult::Sprite(result) => Ok(AnimTreeUpdate {
                    time: result.delay,
                    keyframe_index: result.keyframe_index,
                    atlas_index: result.atlas_index,
                })
            },
            AnimNode::MatchNode(node) => match node.execute(state) {
                Ok(id) => self.handle_update(self.get_node(id), delta_time, time, index, state),
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
        start_node
            .get_id()
            .is_nil()
            .then_some(start_node.set_default_id());
        nodes.insert(start_node.get_id(), start_node);

        let mut instance = Self {
            nodes,
            start_node: Uuid::nil(),
        };

        instance.set_default_start_node();
        instance
    }

    pub fn set_default_start_node(&mut self) -> &mut Self {
        self.start_node = self
            .nodes
            .values()
            .next()
            .and_then(|x| Some(x.get_id()))
            .unwrap_or(Uuid::nil());
        self
    }

    pub fn set_start_node(&mut self, uuid: &Uuid) -> Result<&mut Self, &str> {
        const NO_UUID_EXIST_ERR: &str = "No uuid exist with this uuid";

        match self.nodes.get(&uuid) {
            Some(node) => {
                self.start_node = node.get_id();
                Ok(self)
            }
            None => Err(NO_UUID_EXIST_ERR),
        }
    }

    pub fn iter(&self) -> Values<'_, Uuid, AnimNode<T>> {
        self.nodes.values().into_iter()
    }
}

impl<T> AnimTreeWrap<T> for AnimTree<T>
where
    T: AnimState,
{
    fn get(&self) -> &AnimTree<T> {
        self
    }
}
