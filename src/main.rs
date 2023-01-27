use bevy::{prelude::*, window::PresentMode};
use plugins::base::BasePlugin;
use plugins::physics::PhysicsPlugin;
use plugins::tilemap::TilemapPlugin;

mod entities;
mod plugins;

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
        .run();
}
