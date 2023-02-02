use crate::data::clean_up::Persist;

use super::display::DisplaySettings;
use bevy::{
    prelude::*,
    window::{WindowDescriptor, WindowPlugin},
    DefaultPlugins,
};
pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let default_settings = DisplaySettings::default();
        let res = default_settings.resolution.to_array();

        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
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
                })
                .set(ImagePlugin::default_nearest()),
        )
        // .add_plugin(WorldInspectorPlugin)
        .add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(Persist)
        .insert(Name::from("Camera"));
}
