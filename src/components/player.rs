use bevy::prelude::*;

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