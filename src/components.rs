use bevy::prelude::Plugin;

use self::player::*;

pub mod player;

pub struct ComponentsPlugin;
impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        player::register(app);
    }
}