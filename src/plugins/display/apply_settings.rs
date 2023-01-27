use bevy::{prelude::*};

use super::{DisplayMode, DisplayResolution, DisplayVsync};

pub fn apply_mode (
    window: &mut Window,
    mode: &DisplayMode
) {
    window.set_mode(mode.to_bevy_mode());
}

pub fn apply_resolution (
    window: &mut Window,
    res: &DisplayResolution
) {
    let res_arr = res.to_array();
    window.set_resolution(res_arr[0], res_arr[1]);
}

pub fn apply_vsync(
    window: &mut Window,
    vsync: &DisplayVsync
) {
    window.set_present_mode(vsync.to_bevy_presentmode());
}