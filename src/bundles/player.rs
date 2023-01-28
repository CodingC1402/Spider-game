use bevy::prelude::*;

use crate::components::player::{Player, PlayerMovement, PlayerJump, PlayerInfo};

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