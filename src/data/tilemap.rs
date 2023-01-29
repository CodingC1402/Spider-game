use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkIntCell;

#[derive(Debug, Default, Reflect, Component)]
pub struct Platform;

// Add ground, walls, etc component later;


#[derive(Component, Default)]
pub struct TerrainTile;

#[derive(Component, Default)]
pub struct WebStickerTile;

#[derive(Component, Default)]
pub struct TrapTile;

// Components attached to connected colliders

#[derive(Component, Default)]
pub struct Terrain;

#[derive(Component, Default)]
pub struct Trap;

#[derive(Component, Default)]
pub struct WebStickable;

#[derive(Component, Default)]
pub struct NonStickable;

#[derive(Bundle, Default, LdtkIntCell)]
pub struct TerrainTileBundle {
    tile: TerrainTile,
}

#[derive(Bundle, Default, LdtkIntCell)]
pub struct WebStickerTileBundle {
    web_sticker: WebStickerTile,
}

#[derive(Bundle, Default, LdtkIntCell)]
pub struct TrapTileBundle {
    trap: TrapTile,
}

#[derive(Bundle, Default, LdtkIntCell)]
pub struct TerrainBundle {
    tile: Terrain,
    platform: Platform,
    non_stick: NonStickable,
}

#[derive(Bundle, Default, LdtkIntCell)]
pub struct WebStickerBundle {
    web_sticker: WebStickable,
    platform: Platform,
}

#[derive(Bundle, Default, LdtkIntCell)]
pub struct TrapBundle {
    trap: Trap,
    non_stick: NonStickable,
}