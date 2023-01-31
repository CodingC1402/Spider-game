use std::collections::HashMap;
use bevy::{prelude::*, utils::Uuid};

use crate::derive::*;

pub struct NodeComponentData {
    pub next: bool,
}

/// Has to be in the same entity that contain the animation controller
#[derive(Component)]
pub struct ControlNodeComponent {
    pub data: HashMap<Uuid, NodeComponentData>,
}

#[derive(ToUuid, Default)]
pub struct ComponentNode {
    pub id: Uuid,
    pub next: Uuid,
}

impl ComponentNode {
    pub fn new(next: Uuid) -> Self {
        let mut instance = Self {
            id: Uuid::nil(),
            next
        };

        instance.id = instance.to_uuid();
        instance
    }
}