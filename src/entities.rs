use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Default)]
pub struct Terrain;

#[derive(Bundle, Default, LdtkIntCell)]
pub struct TerrainBundle {
    tile: Terrain,
}

#[derive(Component, Default)]
pub struct WebSticker;

#[derive(Bundle, Default, LdtkIntCell)]
pub struct WebStickerBundle {
    web_sticker: WebSticker,
}

#[derive(Component, Default)]
pub struct Trap;

#[derive(Bundle, Default, LdtkIntCell)]
pub struct TrapBundle {
    trap: Trap,
}
