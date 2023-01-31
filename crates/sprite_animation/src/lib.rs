use bevy::prelude::*;

pub mod animation;
pub mod collection;
pub mod controller;
pub mod tree;
pub mod prelude;
pub mod nodes;
pub mod utils;
mod system;
pub mod derive;

pub struct SpriteAnimationPlugin;
impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}