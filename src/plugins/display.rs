use bevy::prelude::{Plugin, Resource};

use settings_events::apply_settings_system;

pub struct WindowPlugin;

mod settings_events;

#[derive(Default, Debug)]
pub enum DisplayMode {
    #[default]
    FullScreen,
    BorderlessWindow,
    Windowed
}

#[derive(Resource, Default, Debug)]
pub struct DisplaySettings {
    width: f32,
    height: f32,
    vsync: bool,
    mode: DisplayMode
}

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .insert_resource(DisplaySettings::default())
        .add_system(apply_settings_system);
    }
}

