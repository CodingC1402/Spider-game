use bevy::{prelude::{Plugin, PluginGroup}, DefaultPlugins, window::{WindowPlugin, WindowDescriptor, PresentMode}};

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.
        add_plugins(DefaultPlugins.set(WindowPlugin {
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
        }));
    }
}