use std::time::Duration;

use crate::{data::tilemap::*, utils::state_helper::StateExtend, GameState};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod modify;
mod spawn;

const MAP_PATH: &str = "ldtk.ldtk";
const START_LEVEL: usize = 0;
const LEVELS: &[&str] = &[
    "Level_0", "Level_1", "Level_2", "Level_3", "Level_4", "Level_5",
];
// layers
const DARK_CAVE_LAYER: &str = "DarkCave";
const SPIKES_LAYER: &str = "Spikes";
const SURFACE_LAYER: &str = "Surface";
// dark cave
const DARK_TERRAIN: i32 = 1;
const PINK_TERRAIN: i32 = 2;
const METAL: i32 = 3;
// spikes
const SPIKE_UP: i32 = 1;
const SPIKE_DOWN: i32 = 2;
const SPIKE_LEFT: i32 = 3;
const SPIKE_RIGHT: i32 = 4;
// surface
const GRASS: i32 = 1;
const DIRT: i32 = 3;
const EDGE: i32 = 4;
const CREDITS_SENSOR: i32 = 5;
const EDGE_SENSOR: i32 = 6;
const CREDITS: &str = "Credits";
pub const COIN: &str = "Coin";

pub const TILE_HALF_SIZE: (f32, f32) = (4.0, 4.0);
const TEXT_FONT_PATH: &str = "ThaleahFat.ttf";
const CREDITS_LEVEL_INDEX: usize = LEVELS.len() - 1;

#[derive(Resource, Default)]
pub struct FontHandle(Handle<Font>);

pub struct LevelChanged {
    previous: usize,
    current: usize,
}

pub struct TilemapPlugin {
    run_in: Option<GameState>,
}

impl TilemapPlugin {
    pub fn new(state: Option<GameState>) -> Self {
        Self { run_in: state }
    }
}

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
            .insert_resource(LevelSelection::Identifier(String::from(
                LEVELS[START_LEVEL],
            )))
            .insert_resource(FontHandle::default())
            .insert_resource(modify::CreditTimer {
                timer: Timer::new(Duration::from_secs_f32(1.25), TimerMode::Repeating),
                line_count: 0,
                active: false,
            })
            .add_event::<LevelChanged>()
            // ldtk incels
            .register_ldtk_int_cell_for_layer::<TerrainTileBundle>(DARK_CAVE_LAYER, DARK_TERRAIN)
            .register_ldtk_int_cell_for_layer::<TerrainTileBundle>(DARK_CAVE_LAYER, PINK_TERRAIN)
            .register_ldtk_int_cell_for_layer::<WebStickerTileBundle>(DARK_CAVE_LAYER, METAL)
            .register_ldtk_int_cell_for_layer::<TrapTileBundle>(SPIKES_LAYER, SPIKE_UP)
            .register_ldtk_int_cell_for_layer::<TrapTileBundle>(SPIKES_LAYER, SPIKE_DOWN)
            .register_ldtk_int_cell_for_layer::<TrapTileBundle>(SPIKES_LAYER, SPIKE_LEFT)
            .register_ldtk_int_cell_for_layer::<TrapTileBundle>(SPIKES_LAYER, SPIKE_RIGHT)
            .register_ldtk_int_cell_for_layer::<TerrainTileBundle>(SURFACE_LAYER, GRASS)
            .register_ldtk_int_cell_for_layer::<TerrainTileBundle>(SURFACE_LAYER, DIRT)
            .register_ldtk_int_cell_for_layer::<EdgeBundle>(SURFACE_LAYER, EDGE)
            .register_ldtk_int_cell_for_layer::<CreditsSensorBundle>(SURFACE_LAYER, CREDITS_SENSOR)
            .register_ldtk_int_cell_for_layer::<EdgeSensorBundle>(SURFACE_LAYER, EDGE_SENSOR)
            // ldtk entities
            .register_ldtk_entity::<CoinBundle>(COIN)
            .register_ldtk_entity::<CreditsBundle>(CREDITS)
            // systems
            .add_startup_system_if_state(self.run_in, setup)
            .add_startup_system_if_state(self.run_in, spawn::spawn_tilemap)
            .add_system_run_if(self.run_in, spawn::spawn_tile_colliders)
            .add_system_run_if(self.run_in, modify::update_level_selection)
            .add_system_run_if(self.run_in, modify::camera_fit_inside_current_level)
            .add_system_run_if(self.run_in, modify::collect_coin)
            .add_system_run_if(self.run_in, modify::spawn_credits)
            .add_system_run_if(self.run_in, modify::spawn_surface_edges)
            .add_system_run_if(self.run_in, modify::update_credit_timer);
    }
}

fn setup(asset_server: Res<AssetServer>, mut font_handle: ResMut<FontHandle>) {
    font_handle.0 = asset_server.load(TEXT_FONT_PATH);
}

pub fn current_level_index(level_selection: &LevelSelection) -> Option<usize> {
    if let LevelSelection::Identifier(ref id) = *level_selection {
        LEVELS.iter().position(|level| *level == id)
    } else {
        None
    }
}

pub fn entered_credits(mut evr_level_changed: EventReader<LevelChanged>) -> bool {
    transitioned_from_to(&mut evr_level_changed, None, Some(CREDITS_LEVEL_INDEX))
}

pub fn exited_credits(mut evr_level_changed: EventReader<LevelChanged>) -> bool {
    transitioned_from_to(&mut evr_level_changed, Some(CREDITS_LEVEL_INDEX), None)
}

pub fn transitioned_from_to(
    evr_level_changed: &mut EventReader<LevelChanged>,
    from_level: Option<usize>,
    to_level: Option<usize>,
) -> bool {
    evr_level_changed.iter().any(|ev| {
        let LevelChanged { previous, current } = *ev;
        if from_level.is_none() && to_level.is_none() {
            false
        } else if from_level.is_some() && to_level.is_some() {
            previous == from_level.unwrap() && current == to_level.unwrap()
        } else if from_level.is_none() {
            current == to_level.unwrap()
        } else {
            previous == from_level.unwrap()
        }
    })
}
