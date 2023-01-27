use bevy::{prelude::{Plugin, Resource}, window::{WindowMode, PresentMode}};

use self::handle_keypress::handle_keypress;

pub struct DisplayPlugin;

mod apply_settings;
mod handle_keypress;

pub enum DisplaySettingEvent {
    ModeChanged(DisplayMode),
    ResolutionChanged(DisplayResolution),
    VsyncChanged(DisplayVsync),
}

#[derive(Default, Debug, Clone)]
pub enum DisplayMode {
    FullScreen,
    BorderlessWindow,
    #[default]
    Windowed,
}
impl DisplayMode {
    pub fn to_bevy_mode(&self) -> WindowMode {
        match self {
            DisplayMode::FullScreen => WindowMode::Fullscreen,
            DisplayMode::BorderlessWindow => WindowMode::BorderlessFullscreen,
            DisplayMode::Windowed => WindowMode::Windowed,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum DisplayResolution {
    Res1920x1080,
    #[default]
    Res1280x720,
    Res800x600,
}
impl DisplayResolution {
    pub fn to_array(&self) -> &[f32; 2] {
        const OPTIONS: [[f32; 2]; 3] = [[1920.0, 1080.0], [1280.0, 720.0], [800.0, 600.0]];
        &OPTIONS[*self as usize]
    }
}

#[derive(Default, Debug, Clone)]
pub enum DisplayVsync {
    #[default] On,
    Off
}
impl DisplayVsync {
    pub fn to_bevy_presentmode(&self) -> PresentMode {
        match self {
            DisplayVsync::On => PresentMode::AutoVsync,
            DisplayVsync::Off => PresentMode::AutoNoVsync
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct DisplaySettings {
    pub resolution: DisplayResolution,
    pub vsync: DisplayVsync,
    pub mode: DisplayMode,
}
impl Default for DisplaySettings {
    fn default() -> Self {
        Self { 
            resolution: Default::default(), 
            vsync: Default::default(), 
            mode: Default::default() 
        }
    }
}

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<DisplaySettingEvent>()
            .insert_resource(DisplaySettings::default())
            .add_system(handle_keypress);
    }
}