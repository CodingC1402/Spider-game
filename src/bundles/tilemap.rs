use crate::components::tilemap::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

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
