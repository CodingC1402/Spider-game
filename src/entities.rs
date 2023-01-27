use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Default)]
pub struct Tile;

#[derive(Bundle, Default, LdtkIntCell)]
pub struct TileBundle {
    tile: Tile,
}
