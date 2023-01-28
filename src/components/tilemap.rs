use bevy::prelude::*;

#[derive(Debug, Default, Reflect, Component)]
pub struct Platform;

// Components spawned for each tile in map

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
pub struct WebSticker;

#[derive(Component, Default)]
pub struct Trap;
