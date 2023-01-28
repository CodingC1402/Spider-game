use crate::bundles::player::PlayerBundle;

use super::display::DisplaySettings;
use bevy::{
    prelude::*,
    window::{WindowDescriptor, WindowPlugin},
    DefaultPlugins, render::camera::ScalingMode,
};
pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let default_settings = DisplaySettings::default();
        let res = default_settings.resolution.to_array();

        app
        .add_plugins(DefaultPlugins.set(WindowPlugin {
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
        }).set(ImagePlugin::default_nearest()))
        // .add_plugin(WorldInspectorPlugin)
        .add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    let camera = Camera2dBundle {
        transform: Transform::from_xyz(128.0, 128.0, 1000.0),
        projection: OrthographicProjection {
            scale: 0.3,
            ..Default::default()
        },
        ..Default::default()
    };
    
    commands
        .spawn(camera)
        .insert(Name::from("Camera"));
}