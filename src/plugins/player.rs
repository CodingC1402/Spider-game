mod jump;
mod movement;

use bevy::prelude::*;

use crate::prefabs::player::spawn_player;

use self::{
    jump::{check_if_grounded, handle_jump},
    movement::handle_movement,
};

pub enum PlayerEvent {
    Airborn(Entity),
    Jumped(Entity),
    Grounded(Entity),
    Hurted(Entity),
    Attacks(Entity),
    Moving(Entity),
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
            jump: KeyCode::X,
            left: KeyCode::Left,
            right: KeyCode::Right,
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
            .add_system(handle_movement);
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
