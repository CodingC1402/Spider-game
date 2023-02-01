use bevy::utils::Uuid;

use crate::prelude::AnimState;

use super::{Node, NodeResult};

pub struct AnyNode {}
impl<T> Node<T> for AnyNode
where
    T: AnimState,
{
    fn execute(
        &self,
        _data: &crate::prelude::AnimData<T>,
        _delta_time: f32,
        _logic_stack: &mut Vec<(Uuid, usize)>,
    ) -> super::NodeResult {
        NodeResult::Err(String::from(""))
    }
}
