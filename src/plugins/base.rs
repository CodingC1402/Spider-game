use bevy::{prelude::{Plugin, PluginGroup}, DefaultPlugins, window::{WindowPlugin, WindowDescriptor}};

use super::display::DisplaySettings;

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let default_settings = DisplaySettings::default();
        let res = default_settings.resolution.to_array();

        app.
        add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: res[0],
                height: res[1],
                title: "Spider game".to_string(),
                cursor_visible: true,
                decorations: true,
                mode: default_settings.mode.to_bevy_mode(),
                present_mode: default_settings.vsync.to_bevy_presentmode(),
                ..Default::default()
            },
            ..Default::default()
        }));
    }
}