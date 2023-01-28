use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::{Collider, CollisionGroups, Sensor};

use crate::components::{
    physics::{CollisionGroupsFilter, GameCollisionGroups},
    player::*,
};

use super::physics::RigidBodyBundle;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub sprite: SpriteSheetBundle,
    pub physics: RigidBodyBundle,
    pub player: Player,
    pub jump: PlayerJump,
    pub info: PlayerInfo,
    pub movement: PlayerMovement,
    pub name: Name,
}

#[derive(Bundle)]
pub struct PlayerFootBundle {
    pub transform: TransformBundle,
    pub collider: Collider,
    pub sensor: Sensor,
    pub foot_comp: PlayerFoot,
    pub name: Name,
    pub collision_groups: CollisionGroups,
}

impl Default for PlayerFootBundle {
    fn default() -> Self {
        Self {
            transform: TransformBundle::default(),
            collider: Collider::default(),
            sensor: Sensor,
            foot_comp: PlayerFoot::default(),
            name: Name::default(),
            collision_groups: CollisionGroups {
                memberships: GameCollisionGroups::PLAYER,
                filters: GameCollisionGroups::PLAYER.filter_group(),
            },
        }
    }
}
