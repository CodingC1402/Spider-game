use std::collections::HashMap;
use std::hash::Hash;

use crate::derive::*;
use crate::prelude::AnimState;
use bevy::prelude::warn;
use bevy::utils::Uuid;

#[derive(Default)]
pub struct MatchNode<T>
where
    T: Hash + Eq + Default,
{
    pub id: Uuid,
    pub pair: HashMap<T, Uuid>,
}
impl<T> MatchNode<T>
where
    T: AnimState,
{
    fn get_fallback(&self) -> Option<Uuid> {
        self.pair.values().next().and_then(|id| Some(*id))
    }
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
        instance.id = instance.to_uuid();

        instance
    }

    pub fn execute(&self, state: &T) -> Result<Uuid, String> {
        self.pair
            .get(state)
            .and_then(|id| Some(*id))
            .or_else(|| {
                self.get_fallback().and_then(|id| {
                    warn!(
                        "Can't find node id for state {}, falling back to {}",
                        state.to_string(),
                        id
                    );
                    Some(id)
                })
            })
            .ok_or(format!("Can't find node id for state {} and failed to fallback to another node", state.to_string()))
    }
}
