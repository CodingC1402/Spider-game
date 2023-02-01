use std::hash::Hash;

use bevy::{utils::Uuid};

use crate::{utils::extra_uuid::ToUuid, prelude::{AnimData, AnimState}};

use self::{play_node::PlayNode, match_node::MatchNode, all_node::AllNode};

pub mod play_node;
pub mod component_node;
pub mod match_node;
pub mod all_node;
pub mod any_node;

pub trait Node<T> where T : AnimState {
    fn execute(&self, data: &AnimData<T>, delta_time: f32, logic_stack: &mut Vec<(Uuid, usize)>) -> NodeResult;
}
pub enum NodeResult {
    /// delay [f32], keyframe index [usize], atlas_index [usize], current_node [Uuid]
    Sprite(f32, usize, usize, Uuid),
    Node(Uuid),
    /// node [Uuid], top to add back later ([Uuid], [usize])
    LogicNode(Uuid, (Uuid, usize)),
    /// Finish playing animation
    Finished,
    NoUpdate,
    Err(String)
}

pub enum AnimNode<T> where T : Hash + Eq + Default {
    PlayNode(PlayNode),
    MatchNode(MatchNode<T>),
    AllNode(AllNode)
}
impl<T> Node<T> for AnimNode<T> where T: AnimState {
    fn execute(&self, data: &AnimData<T>, delta_time: f32, logic_stack: &mut Vec<(Uuid, usize)>) -> NodeResult {
        match self {
            AnimNode::PlayNode(node) => node.execute(data, delta_time, logic_stack),
            AnimNode::MatchNode(node) => node.execute(data, delta_time, logic_stack),
            AnimNode::AllNode(node) => node.execute(data, delta_time, logic_stack),
        }
    }
}

impl<T> AnimNode<T> where T : Hash + Eq + Default {
    pub fn get_id(&self) -> Uuid {
        match self {
            AnimNode::PlayNode(inner) => inner.id,
            AnimNode::MatchNode(inner) => inner.id,
            AnimNode::AllNode(inner) => inner.id,
        }
    }

    pub fn set_default_id(&mut self) -> Uuid {
        match self {
            AnimNode::PlayNode(inner) => {
                inner.id = inner.new_uuid();
                inner.id
            },
            AnimNode::MatchNode(inner) => {
                inner.id = inner.new_uuid();
                inner.id
            },
            AnimNode::AllNode(inner) => {
                inner.id = inner.new_uuid();
                inner.id
            }
        }
    }
}