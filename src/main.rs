use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use iyes_loopless::prelude::AppLooplessStateExt;
use plugins::{
    base::BasePlugin, debug::DebugPlugin, display::DisplayPlugin, physics::PhysicsPlugin,
    player::PlayerPlugin, tilemap::TilemapPlugin, clean_up::CleanUpPlugin,
};
use strum::EnumIter;

mod data;
mod plugins;
mod utils;

#[derive(Hash, Debug, Eq, PartialEq, Clone, Copy, EnumIter)]
pub enum GameState {
    InMenu,
    InGame,
}

fn main() {
    let mut app = App::new();

    app
        // Resources
        .add_loopless_state(GameState::InMenu)
        .insert_resource(ClearColor(Color::Rgba {
            red: 0.2,
            green: 0.2,
            blue: 0.3,
            alpha: 1.,
        }))
        // Systems, testing purpose
        // Plugins
        .add_plugin(BasePlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(DisplayPlugin)

        // This will also clean up debug line, I can't do anything about that
        // because bevy decided that the component used to mark that is pub(crate)
        .add_plugin(CleanUpPlugin::<GameState>::default())
        .add_plugin(TilemapPlugin::new(Some(GameState::InGame)))
        .add_plugin(PlayerPlugin::new(Some(GameState::InGame)));

    #[cfg(debug_assertions)]
    app.add_plugin(WorldInspectorPlugin).add_plugin(DebugPlugin);

    app.run();
}
