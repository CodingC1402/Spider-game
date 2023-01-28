use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use components::ComponentsPlugin;
use plugins::{
    base::BasePlugin, display::DisplayPlugin, physics::PhysicsPlugin, player::PlayerPlugin,
    tilemap::TilemapPlugin,
};

mod bundles;
mod components;
mod plugins;
mod prefabs;

fn main() {
    App::new()
        // Resources
        .insert_resource(ClearColor(Color::Rgba {
            red: 0.2,
            green: 0.2,
            blue: 0.3,
            alpha: 1.,
        }))
        // Systems
        // Plugins
        .add_plugin(BasePlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(DisplayPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ComponentsPlugin)
        .add_plugin(WorldInspectorPlugin)
        .run();
}
