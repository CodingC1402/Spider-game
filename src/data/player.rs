use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use sprite_animation::prelude::AnimData;

use crate::plugins::player::PlayerAnimState;

use super::physics::*;

pub fn register(app: &mut bevy::prelude::App) {
    app
    .register_type::<PlayerMovement>()
    .register_type::<Player>()
    .register_type::<PlayerJump>()
    .register_type::<PlayerInfo>();
}

#[derive(Component, Default, Reflect)]
pub struct Player;
#[derive(Component, Default, Reflect)]
pub struct PlayerMovement {
    pub airborne_acceleration: f32,
    pub acceleration: f32,
    pub landing_accel: f32,
    /// When reach max velocity, force will stop being applied until velocity is lower than
    /// max again
    pub max_velocity: f32,
    /// Velocity when changing direction or stop moving to create a sense of slowing down then switch
    /// direction
    pub decelerate_velocity: f32,
    pub stop_velocity: f32,
    pub axis: f32,
}
#[derive(Component, Default, Reflect)]
pub struct PlayerJump {
    pub strength: f32,
    pub air_upward_force: f32,

    // After this duration system will stop refresh vec2
    pub duration: f32,
    pub counter: f32,
}
#[derive(Component, Default, Reflect)]
pub struct PlayerInfo {
    pub is_grounded: bool,
}
#[derive(Component, Default, Reflect)]
pub struct PlayerFoot;
#[derive(Component, Default, Reflect)]
pub struct PlayerHead;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub sprite: SpriteSheetBundle,
    pub physics: RigidBodyBundle,
    pub player: Player,
    pub jump: PlayerJump,
    pub info: PlayerInfo,
    pub movement: PlayerMovement,
    pub name: Name,
    pub anim: AnimData<PlayerAnimState>
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

#[derive(Bundle, Default)]
pub struct PlayerHeadBundle {
    pub transform: TransformBundle,
    pub collider: Collider,
    pub sensor: Sensor,
    pub head_comp: PlayerHead,
    pub name: Name
}