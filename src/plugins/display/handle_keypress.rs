use bevy::{prelude::{EventWriter, Input, KeyCode, Res, ResMut}, window::Windows};

use super::{DisplayMode, DisplaySettingEvent, DisplaySettings, apply_settings::{apply_mode, apply_vsync, apply_resolution}, DisplayResolution, DisplayVsync};

pub fn handle_keypress (
    keyboard: Res<Input<KeyCode>>,
    mut settings: ResMut<DisplaySettings>,
    mut e_writer: EventWriter<DisplaySettingEvent>,
    mut windows: ResMut<Windows>
) {
    let window = windows.primary_mut();

    keyboard
        .just_pressed(KeyCode::F11)
        .then(|| {
            apply_mode(window, change_mode_setting(&mut settings.as_mut()));
            e_writer.send(DisplaySettingEvent::ModeChanged(settings.mode.clone()));
        });

    keyboard
        .just_pressed(KeyCode::F10)
        .then(|| {
            apply_vsync(window, change_vsync_setting(&mut settings.as_mut()));
            e_writer.send(DisplaySettingEvent::VsyncChanged(settings.vsync.clone()));
        });

    keyboard
        .just_pressed(KeyCode::F12)
        .then(|| {
            apply_resolution(window, change_resolution_setting(&mut settings.as_mut())); 
            e_writer.send(DisplaySettingEvent::ResolutionChanged(settings.resolution.clone()));
        });
}

fn change_mode_setting(settings: & mut DisplaySettings) -> &DisplayMode {
    settings.mode = match settings.mode {
        DisplayMode::FullScreen => DisplayMode::BorderlessWindow,
        DisplayMode::BorderlessWindow => DisplayMode::Windowed,
        DisplayMode::Windowed => DisplayMode::FullScreen,
    };

    &settings.mode
}

fn change_vsync_setting(settings: &mut DisplaySettings) -> &DisplayVsync {
    settings.vsync = match settings.vsync {
        DisplayVsync::On => DisplayVsync::Off,
        DisplayVsync::Off => DisplayVsync::On,
    };

    &settings.vsync
}

fn change_resolution_setting(settings: &mut DisplaySettings) -> &DisplayResolution{
    settings.resolution = match settings.resolution {
        DisplayResolution::Res1920x1080 => DisplayResolution::Res1280x720,
        DisplayResolution::Res1280x720 => DisplayResolution::Res800x600,
        DisplayResolution::Res800x600 => DisplayResolution::Res1920x1080,
    };

    &settings.resolution
}
