use crate::data::clean_up::Persist;

use super::display::DisplaySettings;
use bevy::{
    prelude::*,
    window::{WindowDescriptor, WindowPlugin, WindowId},
    DefaultPlugins, winit::WinitWindows,
};
use winit::window::Icon;
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


fn setup(mut commands: Commands,  windows: NonSend<WinitWindows>) {
    let primary = windows.get_window(WindowId::primary()).unwrap();

    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
    primary.set_window_icon(Some(icon));

    commands
        .spawn(Camera2dBundle::default())
        .insert(Persist)
        .insert(Name::from("Camera"));
}