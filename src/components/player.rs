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
    pub speed: f32,
    pub damping: f32,
}

#[derive(Component, Default, Reflect)]
pub struct PlayerJump {
    pub strength: f32,
    pub duration: f32,
    pub damping: f32,
}

#[derive(Component, Default, Reflect)]
pub struct PlayerInfo {
    pub is_grounded: bool,
}

#[derive(Component, Default, Reflect)]
pub struct PlayerFoot;