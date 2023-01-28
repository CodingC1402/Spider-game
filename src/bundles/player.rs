use bevy::prelude::*;
use bevy_rapier2d::prelude::{Sensor, Collider};

use crate::components::player::{Player, PlayerMovement, PlayerJump, PlayerInfo, PlayerFoot, PlayerHead};

use super::physics::RigidBodyBundle;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub sprite: SpriteSheetBundle,
    pub physic: RigidBodyBundle,
    pub player: Player,
    pub jump: PlayerJump,
    pub info: PlayerInfo,
    pub movement: PlayerMovement,
    pub name: Name,
}

#[derive(Bundle, Default)]
pub struct PlayerFootBundle {
    pub transform: TransformBundle,
    pub collider: Collider,
    pub sensor: Sensor,
    pub foot_comp: PlayerFoot,
    pub name: Name
}

#[derive(Bundle, Default)]
pub struct PlayerHeadBundle {
    pub transform: TransformBundle,
    pub collider: Collider,
    pub sensor: Sensor,
    pub head_comp: PlayerHead,
    pub name: Name
}