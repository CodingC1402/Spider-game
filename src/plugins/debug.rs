use bevy::prelude::*;
use bevy_rapier2d::render::RapierDebugRenderPlugin;
use iyes_loopless::state::{CurrentState, NextState};

use crate::GameState;

// Add debug codes, systems without spreading it everywhere

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        #[cfg(debug_assertions)]
        app.add_system(switch_state)
        .add_plugin(RapierDebugRenderPlugin::default());
    }
}

fn switch_state(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    current_state: Res<CurrentState<GameState>>,
) {
    if !input.just_pressed(KeyCode::Escape) {
        return;
    }

    let next_state = NextState(match current_state.0 {
        GameState::InMenu => GameState::InGame,
        GameState::InGame => GameState::InMenu,
    });
    info!("Changing to state {:?}", next_state.0);
    commands.insert_resource(next_state);
}
