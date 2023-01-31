use std::hash::Hash;

use bevy::utils::Uuid;

use crate::utils::extra_uuid::ToUuid;

use self::{play_node::PlayNode, match_node::MatchNode, component_node::ComponentNode};

pub mod play_node;
pub mod component_node;
pub mod match_node;

pub enum AnimNode<T> where T : Hash + Eq + Default {
    PlayNode(PlayNode),
    MatchNode(MatchNode<T>),
    ComponentNode(ComponentNode)
}

impl<T> AnimNode<T> where T : Hash + Eq + Default {
    pub fn get_id(&self) -> Uuid {
        match self {
            AnimNode::PlayNode(inner) => inner.id,
            AnimNode::MatchNode(inner) => inner.id,
            AnimNode::ComponentNode(inner) => inner.id,
        }
    }

    pub fn set_default_id(&mut self) -> Uuid {
        match self {
            AnimNode::PlayNode(inner) => {
                inner.id = inner.to_uuid();
                inner.id
            },
            AnimNode::MatchNode(inner) => {
                inner.id = inner.to_uuid();
                inner.id
            },
            AnimNode::ComponentNode(inner) => {
                inner.id = inner.to_uuid();
                inner.id
            }
        }
    }
}