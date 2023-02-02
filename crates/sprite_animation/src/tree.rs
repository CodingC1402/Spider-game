use std::{
    any::type_name,
    collections::{hash_map::Values, HashMap},
    hash::Hash,
    ops::Deref,
};

use crate::prelude::{Node, *};
use bevy::{prelude::*, utils::Uuid};

pub trait AnimTreeWrap<T>: Resource
where
    T: AnimState,
{
    fn get(&self) -> &AnimTree<T>;
    fn get_mut(&mut self) -> &mut AnimTree<T>;
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

pub enum AnimTreeUpdateResult {
    Update(AnimTreeUpdate),
    Finished,
    NoUpdate,
}
/// This is to return the final result of the update.
pub struct AnimTreeUpdate {
    pub time: f32,
    pub keyframe_index: usize,
    pub atlas_index: usize,
    pub current_node: Uuid,
    pub logic_stack: Vec<(Uuid, usize)>,
}

#[derive(Resource, Default)]
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
        data: &AnimData<T>,
        delta_time: f32,
        mut logic_stack: &mut Vec<(Uuid, usize)>
    ) -> Result<AnimTreeUpdateResult, String> {
        self.handle_update(
            self.get_node(self.start_node),
            data,
            delta_time,
            &mut logic_stack,
        )
    }
    fn handle_update(
        &self,
        node: &AnimNode<T>,
        data: &AnimData<T>,
        delta_time: f32,
        mut logic_stack: &mut Vec<(Uuid, usize)>,
    ) -> Result<AnimTreeUpdateResult, String> {
        match node.execute(data, delta_time, &mut logic_stack) {
            NodeResult::Node(id) => {
                self.handle_update(self.get_node(id), data, delta_time, logic_stack)
            }
            NodeResult::Sprite(delay, index, atlas_index, current_node) => {
                Ok(AnimTreeUpdateResult::Update(AnimTreeUpdate {
                    time: delay,
                    keyframe_index: index,
                    atlas_index,
                    current_node,
                    logic_stack: Vec::new(),
                }))
            }
            NodeResult::Err(str) => Err(str),
            NodeResult::NoUpdate => Ok(AnimTreeUpdateResult::NoUpdate),
            NodeResult::Finished => Ok(AnimTreeUpdateResult::Finished),
            NodeResult::LogicNode(id, top,) => {
                let result = self.handle_update(self.get_node(id), data, delta_time, logic_stack);
                match result {
                    Ok(data) => match data {
                        AnimTreeUpdateResult::NoUpdate | AnimTreeUpdateResult::Update(_) => {
                            logic_stack.push(top);
                            Ok(data)
                        },
                        AnimTreeUpdateResult::Finished => {
                            logic_stack.push((top.0, top.1 + 1));
                            Ok(data)
                        }
                    },
                    Err(str) => Err(str),
                }
            }
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
        id.is_nil().then(|| node.set_default_id());
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

    pub fn insert_unwrap(&mut self, node: AnimNode<T>) -> &mut Self {
        self.insert_node(node).unwrap()
    }

    pub fn new(mut start_node: AnimNode<T>) -> Self {
        let mut nodes = HashMap::new();
        start_node
            .get_id()
            .is_nil()
            .then(|| start_node.set_default_id());
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

    fn get_mut(&mut self) -> &mut AnimTree<T> {
        self
    }
}
