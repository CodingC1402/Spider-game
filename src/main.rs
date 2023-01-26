use bevy::{prelude::*, window::PresentMode};

fn main() {
    App::new()
    // Resources
    .insert_resource(ClearColor(Color::Rgba { red: 0.2, green: 0.2, blue: 0.3, alpha: 1. }))

    // Systems
    

    // Plugins
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            width: 1280.0,
            height: 720.0,
            title: "Spider game".to_string(),
            cursor_visible: true,
            decorations: true,
            present_mode: PresentMode::AutoVsync,
            ..Default::default()
        },
        ..Default::default()
    }))
    .run();
}