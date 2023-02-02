use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Not;

use crate::derive::*;
use crate::prelude::AnimState;
use bevy::utils::Uuid;

use super::{Node, NodeResult};

#[derive(Default)]
pub struct MatchNode<T>
where
    T: Hash + Eq + Default,
{
    pub id: Uuid,
    pub pair: HashMap<T, Uuid>,
    pub default: Uuid
}

impl<T> ToUuid for MatchNode<T> where T: Hash + Eq + Default {}
impl<T> MatchNode<T>
where
    T: AnimState,
{
    pub fn get_next(&self, state: T) -> Option<&Uuid> {
        self.pair.get(&state)
    }

    pub fn insert(&mut self, state: T, uuid: Uuid) -> &mut Self {
        self.pair.insert(state, uuid);
        self
    }

    pub fn new() -> MatchNode<T> {
        let mut instance = MatchNode::default();
        instance.id = instance.new_uuid();

        instance
    }
}

impl<T> Node<T> for MatchNode<T>
where
    T: AnimState,
{
    fn execute(
        &self,
        data: &crate::prelude::AnimData<T>,
        _: f32,
        _: &mut Vec<(Uuid, usize)>,
    ) -> super::NodeResult {
        match self
            .pair
            .get(&data.get_state())
            .and_then(|id| Some(*id))
            .or_else(|| self.default.is_nil().not().then_some(self.default))
            {
            Some(value) => NodeResult::Node(value),
            None => NodeResult::Finished,
        }
    }
}
