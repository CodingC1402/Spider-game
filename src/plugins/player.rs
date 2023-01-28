mod jump;
mod movement;

use bevy::prelude::*;

use crate::prefabs::player::spawn_player;

use self::{jump::{handle_jump, check_if_grounded, check_if_head_bump}, movement::{handle_movement, apply_accel_when_land}};

pub enum PlayerEvent {
    Airborne(Entity),
    Jumped(Entity),
    Grounded(Entity),
    Hurted(Entity),
    Attacks(Entity),
    Moving(Entity),
    Idle(Entity),
    ChangeDirection(Entity)
}

#[derive(Resource, Debug)]
pub struct PlayerControl {
    attack: KeyCode,

    jump: KeyCode,

    left: KeyCode,
    right: KeyCode,
}
impl Default for PlayerControl {
    fn default() -> Self {
        Self {
            attack: KeyCode::C,
            jump: KeyCode::Space,
            left: KeyCode::A,
            right: KeyCode::D,
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(PlayerControl::default())
            .add_event::<PlayerEvent>()
            .add_startup_system(spawn_player_at_start)
            .add_system(handle_jump)
            .add_system(check_if_grounded)
            .add_system(handle_movement)
            .add_system(check_if_head_bump)
            .add_system(apply_accel_when_land);

    }
}

fn spawn_player_at_start(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    spawn_player(
        &mut commands,
        Transform::from_xyz(50.0, 50.0, 900.0),
        asset_server.as_ref(),
        texture_atlases.as_mut(),
    );
}
