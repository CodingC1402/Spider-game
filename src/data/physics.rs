use std::collections::HashMap;

use bevy::prelude::*;
use bevy_ecs_ldtk::EntityInstance;
use bevy_rapier2d::prelude::*;

use crate::plugins::tilemap;

pub struct GameCollisionGroups;

impl GameCollisionGroups {
    pub const PLAYER: Group = Group::GROUP_1;
    pub const NON_STICK_TERRAIN: Group = Group::GROUP_2;
    pub const WEB_STICKABLE_TERRAIN: Group = Group::GROUP_3;
    pub const WEB: Group = Group::GROUP_4;
    pub const TRAP: Group = Group::GROUP_5;
}

pub trait CollisionGroupsFilter {
    fn filter_group(&self) -> Self;
}

impl CollisionGroupsFilter for Group {
    fn filter_group(&self) -> Self {
        match *self {
            GameCollisionGroups::PLAYER => Group::ALL ^ Group::GROUP_1 ^ Group::GROUP_4,
            GameCollisionGroups::NON_STICK_TERRAIN => Group::GROUP_1 | Group::GROUP_4,
            GameCollisionGroups::WEB_STICKABLE_TERRAIN => Group::GROUP_1 | Group::GROUP_4,
            GameCollisionGroups::WEB => Group::GROUP_2 | Group::GROUP_3 | Group::GROUP_5,
            GameCollisionGroups::TRAP => Group::GROUP_1 | Group::GROUP_4,
            _ => Group::ALL,
        }
    }
}

#[derive(Clone, Debug, Default, Bundle)]
pub struct RigidBodyBundle {
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub gravity_scale: GravityScale,
    pub rotation_constraints: LockedAxes,
    pub force: ExternalForce,
    pub impulse: ExternalImpulse,
    pub complex_ef: ComplexExternalForce,
    pub damping: Damping,
}

#[derive(Clone, Debug, Default, Bundle)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub friction: Friction,
    pub collision_groups: CollisionGroups,
    pub active_events: ActiveEvents,
}

impl From<EntityInstance> for ColliderBundle {
    fn from(entity_instance: EntityInstance) -> Self {
        match entity_instance.identifier.as_str() {
            tilemap::COIN => Self {
                collider: Collider::cuboid(tilemap::TILE_HALF_SIZE.0, tilemap::TILE_HALF_SIZE.1),
                ..default()
            },
            _ => Self::default(),
        }
    }
}

#[derive(Component, Clone, Debug, Default)]
pub struct ComplexExternalForce {
    pub forces: HashMap<u8, Vec2>,
}

impl ComplexExternalForce {
    pub fn next_force(&self) -> u8 {
        self.forces.len() as u8
    }

    pub fn composite_force(&self) -> Vec2 {
        self.forces.values().sum()
    }
}
