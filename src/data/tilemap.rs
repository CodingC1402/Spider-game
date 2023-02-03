use crate::{data::physics::ColliderBundle, plugins::tilemap};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::{LdtkEntity, LdtkIntCell};
use bevy_rapier2d::prelude::{Collider, Sensor};

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

#[derive(Component, Default)]
pub struct SurfaceSensor;

#[derive(Component, Default)]
pub struct Edge;

#[derive(Component, Default)]
pub struct Coin;

#[derive(Component, Default)]
pub struct Credits;

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

#[derive(Bundle, Default)]
pub struct SurfaceSensorBundle {
    ssen: SurfaceSensor,
    collider: ColliderBundle,
    sensor: Sensor,
}

impl LdtkIntCell for SurfaceSensorBundle {
    fn bundle_int_cell(
        _: bevy_ecs_ldtk::IntGridCell,
        _: &bevy_ecs_ldtk::prelude::LayerInstance,
    ) -> Self {
        Self {
            collider: ColliderBundle {
                collider: Collider::cuboid(tilemap::TILE_HALF_SIZE.0, tilemap::TILE_HALF_SIZE.1),
                ..default()
            },
            ..default()
        }
    }
}

#[derive(Bundle, Default, LdtkIntCell)]
pub struct EdgeBundle {
    edge: Edge,
}

#[derive(Bundle, Default, LdtkEntity)]
pub struct CoinBundle {
    coin: Coin,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite: SpriteSheetBundle,
    #[from_entity_instance]
    collider: ColliderBundle,
    sensor: Sensor,
}

#[derive(Bundle, Default, LdtkEntity)]
pub struct CreditsBundle {
    credits: Credits,
}
