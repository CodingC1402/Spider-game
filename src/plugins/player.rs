mod animation;
mod jump;
pub mod lifecycle;
mod movement;
mod shoot_web;

use bevy::prelude::*;

use self::{
    animation::{update_animation, PlayerAnimationPlugin},
    jump::{check_if_grounded, check_if_head_bump, handle_jump},
    lifecycle::*,
    movement::{apply_accel_when_land, handle_movement},
    shoot_web::*,
};

#[derive(Eq, Hash, PartialEq, Default, Clone, Copy, Debug)]
pub enum PlayerAnimState {
    #[default]
    Standing,
    Idle,
    Walking,
    MidAir,
    Jumping,
    Landing,
    Hurt,
    None,
}

#[derive(Debug)]
pub enum PlayerEvent {
    Airborne(Entity),
    Jumped(Entity),
    Grounded(Entity),
    Died(Entity),
    Hurt(Entity),
    Attacks(Entity),
    /// Axis [f32], Player [Entity]
    Moving(f32, Entity),
    Standing(Entity),
    ShotWeb,
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

#[derive(Resource, Debug)]
pub struct PlayerSwingDirection(f32);

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(PlayerControl::default())
            .insert_resource(WebTexture::default())
            .add_event::<PlayerEvent>()
            .add_event::<DespawnWebEvent>()
            .add_startup_system(setup_web_texture)
            .add_startup_system(spawn_player_at_start)
            // movements
            .add_system(handle_jump)
            .add_system(check_if_grounded)
            .add_system(handle_movement)
            .add_system(check_if_head_bump)
            .add_system(apply_accel_when_land)
            // animation
            .add_plugin(PlayerAnimationPlugin)
            .add_system(update_animation)
            // shoot web
            .add_system(handle_shoot_web_input)
            .add_system(shoot_web)
            .add_system(handle_web_head_collision)
            .add_system(update_web_string_and_pull_force)
            .add_system(despawn_web)
            .add_system(despawn_web_on_player_death)
            // death
            .add_system(kill_player)
            .add_system(respawn_player_on_death)
            // testing
            .add_system(handle_player_respawn_input)
            .add_system(adjust_player_pos_to_level);
    }
}

fn spawn_player_at_start(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    spawn_player(
        &mut commands,
        Transform::from_xyz(44.0, 428.0, 900.0),
        asset_server.as_ref(),
        texture_atlases.as_mut(),
    );
}
