mod jump;
mod movement;
mod shoot_web;

use bevy::prelude::*;

use crate::prefabs::player::spawn_player;

use self::{
    jump::{check_if_grounded, check_if_head_bump, handle_jump},
    movement::{apply_accel_when_land, handle_movement},
    shoot_web::{
        despawn_overstretched_web, handle_shoot_web_input, handle_web_head_collision,
        setup_web_texture, shoot_web, update_web_string_transform, WebOverstretchedEvent,
        WebTexture,
    },
};

pub enum PlayerEvent {
    Airborne(Entity),
    Jumped(Entity),
    Grounded(Entity),
    Hurted(Entity),
    Attacks(Entity),
    Moving(Entity),
    Idle(Entity),
    ChangeDirection(Entity),
    ShotWeb
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
            .insert_resource(WebTexture::default())
            .add_event::<PlayerEvent>()
            .add_event::<WebOverstretchedEvent>()
            .add_startup_system(setup_web_texture)
            .add_startup_system(spawn_player_at_start)
            // movements
            .add_system(handle_jump)
            .add_system(check_if_grounded)
            .add_system(handle_movement)
            .add_system(check_if_head_bump)
            .add_system(apply_accel_when_land)
            // shoot web
            .add_system(handle_shoot_web_input)
            .add_system(shoot_web)
            .add_system(handle_web_head_collision)
            .add_system(update_web_string_transform)
            .add_system(despawn_overstretched_web);
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
