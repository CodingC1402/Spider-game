use bevy::utils::Uuid;

use super::{Node, NodeResult};
use crate::prelude::{AnimState, ToUuid};

#[derive(ToUuid, Default)]
pub struct AllNode {
    pub id: Uuid,
    pub nodes: Vec<Uuid>,
}
impl AllNode {
    pub fn new() -> Self {
        let mut instance = Self::default();
        instance.id = instance.new_uuid();

        instance
    }
}

impl<T> Node<T> for AllNode
where
    T: AnimState,
{
    fn execute(
        &self,
        _data: &crate::prelude::AnimData<T>,
        _delta_time: f32,
        logic_stack: &mut Vec<(Uuid, usize)>,
    ) -> super::NodeResult {
        let default = (self.id, 0usize);
        let logic = logic_stack.pop().unwrap_or(default);
        let logic = logic
            .0
            .eq(&self.id)
            .then_some(
                logic
                    .1
                    .ge(&self.nodes.len())
                    .then(|| {
                        logic_stack.clear();
                        default
                    })
                    .unwrap_or((self.id, logic.1)),
            )
            .unwrap_or_else(|| {
                logic_stack.clear();
                default
            });

        NodeResult::LogicNode(
            self.nodes
                .get(logic.1)
                .ok_or(Err::<Uuid, String>(String::from(format!(
                    "AllNode can't find node with index {}, out of {}",
                    logic.1,
                    self.nodes.len()
                ))))
                .unwrap()
                .clone(),
            logic
        )
    }
}
