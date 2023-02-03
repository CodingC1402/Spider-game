mod animation;
mod jump;
pub mod lifecycle;
mod movement;
mod shoot_web;

use bevy::prelude::*;

use crate::{utils::state_helper::StateExtend, GameState};

use self::{
    animation::PlayerAnimationPlugin,
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
    // Hurt(Entity),
    // Attacks(Entity),
    /// Axis [f32], Player [Entity]
    Moving(f32, Entity),
    Standing(Entity),
    ShotWeb,
    Collided(Entity),
}

#[derive(Resource, Debug)]
pub struct PlayerControl {
    // attack: KeyCode,
    jump: KeyCode,

    left: KeyCode,
    right: KeyCode,
}
impl Default for PlayerControl {
    fn default() -> Self {
        Self {
            // attack: KeyCode::C,
            jump: KeyCode::Space,
            left: KeyCode::A,
            right: KeyCode::D,
        }
    }
}

#[derive(Resource, Debug)]
pub struct PlayerSwingDirection(f32);

#[derive(Resource)]
pub struct PlayerPlugin {
    run_in: Option<GameState>,
}
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(PlayerControl::default())
            .insert_resource(WebTexture::default())
            .add_event::<PlayerEvent>()
            .add_event::<DespawnWebEvent>()
            .add_plugin(PlayerAnimationPlugin::new(self.run_in))
            .add_startup_system_if_state(self.run_in, setup_web_texture)
            .add_startup_system_if_state(self.run_in, spawn_player_at_start)
            // movements
            .add_system_run_if(self.run_in, handle_jump)
            .add_system_run_if(self.run_in, check_if_grounded)
            .add_system_run_if(self.run_in, handle_movement)
            .add_system_run_if(self.run_in, check_if_head_bump)
            .add_system_run_if(self.run_in, apply_accel_when_land)
            // shoot web
            .add_system_run_if(self.run_in, handle_shoot_web_input)
            .add_system_run_if(self.run_in, shoot_web)
            .add_system_run_if(self.run_in, handle_web_head_collision)
            .add_system_run_if(self.run_in, update_web_string_and_pull_force)
            .add_system_run_if(self.run_in, despawn_web)
            .add_system_run_if(self.run_in, despawn_web_on_player_death)
            // collision
            .add_system_run_if(self.run_in, player_collision)
            // death
            .add_system_run_if(self.run_in, kill_player)
            .add_system_run_if(self.run_in, respawn_player_on_death)
            // testing
            .add_system_run_if(self.run_in, to_last_level);

        // Add test in the test plugin for easy clean up
    }
}
impl PlayerPlugin {
    pub fn new(run_in: Option<GameState>) -> Self {
        Self { run_in }
    }
}

fn spawn_player_at_start(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    spawn_player(
        &mut commands,
        Transform::from_translation(translation_in_level(0)),
        asset_server.as_ref(),
        texture_atlases.as_mut(),
    );
}
