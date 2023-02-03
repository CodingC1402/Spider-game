use bevy::prelude::Plugin;

pub mod player;
pub mod tilemap;
pub mod physics;
pub mod web;
pub mod clean_up;
pub mod menu;
pub mod cursor;

pub struct ComponentsPlugin;
impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        player::register(app);
    }
}