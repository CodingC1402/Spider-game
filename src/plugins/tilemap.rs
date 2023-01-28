use std::collections::{HashMap, HashSet};

use crate::bundles::tilemap::{
    TerrainBundle, TerrainTileBundle, TrapBundle, TrapTileBundle, WebStickerBundle,
    WebStickerTileBundle,
};
use crate::components::physics::{CollisionGroupsFilter, GameCollisionGroups};
use crate::components::tilemap::{TerrainTile, TrapTile, WebStickerTile};
use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, GridCoords};
use bevy_rapier2d::prelude::*;

const MAP_PATH: &str = "ldtk.ldtk";
const DARK_CAVE_LAYER: &str = "DarkCave";
const SPIKES_LAYER: &str = "Spikes";
const DARK_TERRAIN: i32 = 1;
const PINK_TERRAIN: i32 = 2;
const METAL: i32 = 3;
const SPIKE_UP: i32 = 1;
const SPIKE_DOWN: i32 = 2;
const SPIKE_LEFT: i32 = 3;
const SPIKE_RIGHT: i32 = 4;

const ASPECT_RATIO: f32 = 16. / 9.;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .insert_resource(LdtkSettings {
                set_clear_color: SetClearColor::No,
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                ..default()
            })
            .insert_resource(LevelSelection::Index(0))
            .register_ldtk_int_cell_for_layer::<TerrainTileBundle>(DARK_CAVE_LAYER, DARK_TERRAIN)
            .register_ldtk_int_cell_for_layer::<TerrainTileBundle>(DARK_CAVE_LAYER, PINK_TERRAIN)
            .register_ldtk_int_cell_for_layer::<WebStickerTileBundle>(DARK_CAVE_LAYER, METAL)
            .register_ldtk_int_cell_for_layer::<TrapTileBundle>(SPIKES_LAYER, SPIKE_UP)
            .register_ldtk_int_cell_for_layer::<TrapTileBundle>(SPIKES_LAYER, SPIKE_DOWN)
            .register_ldtk_int_cell_for_layer::<TrapTileBundle>(SPIKES_LAYER, SPIKE_LEFT)
            .register_ldtk_int_cell_for_layer::<TrapTileBundle>(SPIKES_LAYER, SPIKE_RIGHT)
            .add_startup_system(spawn_tilemap)
            .add_system(spawn_tile_colliders)
            .add_system(camera_fit_inside_current_level);
    }
}

fn spawn_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(LdtkWorldBundle {
            ldtk_handle: asset_server.load(MAP_PATH),
            ..default()
        })
        .insert(Name::from("LDtk World"));
}

pub fn spawn_tile_colliders(
    mut commands: Commands,
    terrain_query: Query<(&GridCoords, &Parent), Added<TerrainTile>>,
    metal_query: Query<(&GridCoords, &Parent), Added<WebStickerTile>>,
    trap_query: Query<(&GridCoords, &Parent), Added<TrapTile>>,
    parent_query: Query<
        &Parent,
        (
            Without<TerrainTile>,
            Without<WebStickerTile>,
            Without<TrapTile>,
        ),
    >,
    level_query: Query<(Entity, &Handle<LdtkLevel>)>,
    levels: Res<Assets<LdtkLevel>>,
) {
    let mut level_to_terrain_tile_coords: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();
    let mut level_to_metal_tile_coords: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();
    let mut level_to_trap_tile_coords: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

    terrain_query.for_each(|(&grid_coords, parent)| {
        if let Ok(grandparent) = parent_query.get(parent.get()) {
            level_to_terrain_tile_coords
                .entry(grandparent.get())
                .or_default()
                .insert(grid_coords);
        }
    });
    metal_query.for_each(|(&grid_coords, parent)| {
        if let Ok(grandparent) = parent_query.get(parent.get()) {
            level_to_metal_tile_coords
                .entry(grandparent.get())
                .or_default()
                .insert(grid_coords);
        }
    });
    trap_query.for_each(|(&grid_coords, parent)| {
        if let Ok(grandparent) = parent_query.get(parent.get()) {
            level_to_trap_tile_coords
                .entry(grandparent.get())
                .or_default()
                .insert(grid_coords);
        }
    });
    spawn_connected_colliders::<TerrainBundle>(
        &level_to_terrain_tile_coords,
        &level_query,
        &levels,
        GameCollisionGroups::NON_STICK_TERRAIN,
        &mut commands,
    );
    spawn_connected_colliders::<WebStickerBundle>(
        &level_to_metal_tile_coords,
        &level_query,
        &levels,
        GameCollisionGroups::WEB_STICKABLE_TERRAIN,
        &mut commands,
    );
    spawn_connected_colliders::<TrapBundle>(
        &level_to_trap_tile_coords,
        &level_query,
        &levels,
        GameCollisionGroups::TRAP,
        &mut commands,
    );
}

fn spawn_connected_colliders<T>(
    level_to_tile_coords: &HashMap<Entity, HashSet<GridCoords>>,
    level_query: &Query<(Entity, &Handle<LdtkLevel>)>,
    levels: &Assets<LdtkLevel>,
    tile_collision_groups: Group,
    commands: &mut Commands,
) where
    T: Bundle + Default,
{
    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    level_query.for_each(|(level_entity, level_handle)| {
        if let Some(level_tiles) = level_to_tile_coords.get(&level_entity) {
            let level = levels
                .get(level_handle)
                .expect("Level should be loaded by this point");

            let LayerInstance {
                c_wid: width,
                c_hei: height,
                grid_size,
                ..
            } = level
                .level
                .layer_instances
                .clone()
                .expect("Level asset should have layers")[0];
            let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

            for y in 0..height {
                let mut row_plates: Vec<Plate> = Vec::new();
                let mut plate_start = None;

                for x in 0..width + 1 {
                    match (plate_start, level_tiles.contains(&GridCoords { x, y })) {
                        (Some(s), false) => {
                            row_plates.push(Plate {
                                left: s,
                                right: x - 1,
                            });
                            plate_start = None;
                        }
                        (None, true) => plate_start = Some(x),
                        _ => (),
                    }
                }
                plate_stack.push(row_plates);
            }

            let mut rect_builder: HashMap<Plate, Rect> = HashMap::new();
            let mut prev_row: Vec<Plate> = Vec::new();
            let mut tile_rects: Vec<Rect> = Vec::new();

            plate_stack.push(Vec::new());

            for (y, current_row) in plate_stack.into_iter().enumerate() {
                for prev_plate in &prev_row {
                    if !current_row.contains(prev_plate) {
                        if let Some(rect) = rect_builder.remove(prev_plate) {
                            tile_rects.push(rect);
                        }
                    }
                }

                for plate in &current_row {
                    rect_builder
                        .entry(plate.clone())
                        .and_modify(|e| e.top += 1)
                        .or_insert(Rect {
                            bottom: y as i32,
                            top: y as i32,
                            left: plate.left,
                            right: plate.right,
                        });
                }
                prev_row = current_row;
            }

            commands.entity(level_entity).with_children(|level| {
                for tile_rect in tile_rects {
                    level
                        .spawn_empty()
                        .insert(Collider::cuboid(
                            (tile_rect.right as f32 - tile_rect.left as f32 + 1.)
                                * grid_size as f32
                                / 2.,
                            (tile_rect.top as f32 - tile_rect.bottom as f32 + 1.)
                                * grid_size as f32
                                / 2.,
                        ))
                        .insert(CollisionGroups {
                            memberships: tile_collision_groups,
                            filters: tile_collision_groups.filter_group(),
                        })
                        .insert(RigidBody::Fixed)
                        .insert(Friction::new(1.0))
                        .insert(Transform::from_xyz(
                            (tile_rect.left + tile_rect.right + 1) as f32 * grid_size as f32 / 2.,
                            (tile_rect.bottom + tile_rect.top + 1) as f32 * grid_size as f32 / 2.,
                            0.,
                        ))
                        .insert(GlobalTransform::default())
                        .insert(T::default());
                }
            });
        }
    });
}

pub fn camera_fit_inside_current_level(
    mut camera_query: Query<(
        &mut bevy::render::camera::OrthographicProjection,
        &mut Transform,
    )>,
    level_query: Query<(&Transform, &Handle<LdtkLevel>), Without<OrthographicProjection>>,
    level_selection: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

    for (level_transform, level_handle) in &level_query {
        if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
            let level = &ldtk_level.level;
            if level_selection.is_match(&0, level) {
                let level_ratio = level.px_wid as f32 / ldtk_level.level.px_hei as f32;

                // orthographic_projection.scaling_mode =
                //     bevy::render::camera::ScalingMode::WindowSize;
                // orthographic_projection.scale = 0.5;
                orthographic_projection.scaling_mode = bevy::render::camera::ScalingMode::None;
                orthographic_projection.bottom = 0.;
                orthographic_projection.left = 0.;
                if level_ratio > ASPECT_RATIO {
                    // level is wider than the screen
                    orthographic_projection.top = (level.px_hei as f32 / 9.).round() * 9.;
                    orthographic_projection.right = orthographic_projection.top * ASPECT_RATIO;
                    camera_transform.translation.y = 0.;
                } else {
                    // level is taller than the screen
                    orthographic_projection.right = (level.px_wid as f32 / 16.).round() * 16.;
                    orthographic_projection.top = orthographic_projection.right / ASPECT_RATIO;
                    camera_transform.translation.x = 0.;
                }

                camera_transform.translation.x += level_transform.translation.x;
                camera_transform.translation.y += level_transform.translation.y;
            }
        }
    }
}
