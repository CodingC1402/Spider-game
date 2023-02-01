use bevy::prelude::*;

pub mod collection;
pub mod plugins;
pub mod tree;
pub mod prelude;
pub mod nodes;
pub mod utils;
mod systems;
pub mod derive;

pub struct SpriteAnimationPlugin;
impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}