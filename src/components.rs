use bevy::prelude::Plugin;

pub mod player;
pub mod tilemap;

pub struct ComponentsPlugin;
impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        player::register(app);
    }
}