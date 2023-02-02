use crate::data::tilemap::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod spawn;

const MAP_PATH: &str = "ldtk.ldtk";
const START_LEVEL: usize = 0;
const LEVELS: &[&str] = &["Level_0", "Level_1", "Level_2", "Level_3", "Level_4"];
const DARK_CAVE_LAYER: &str = "DarkCave";
const SPIKES_LAYER: &str = "Spikes";
const DARK_TERRAIN: i32 = 1;
const PINK_TERRAIN: i32 = 2;
const METAL: i32 = 3;
const SPIKE_UP: i32 = 1;
const SPIKE_DOWN: i32 = 2;
const SPIKE_LEFT: i32 = 3;
const SPIKE_RIGHT: i32 = 4;

pub enum TilemapEvent {
    ChangedLevel(usize),
}

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
            .insert_resource(LevelSelection::Identifier(String::from(
                LEVELS[START_LEVEL],
            )))
            .add_event::<TilemapEvent>()
            .register_ldtk_int_cell_for_layer::<TerrainTileBundle>(DARK_CAVE_LAYER, DARK_TERRAIN)
            .register_ldtk_int_cell_for_layer::<TerrainTileBundle>(DARK_CAVE_LAYER, PINK_TERRAIN)
            .register_ldtk_int_cell_for_layer::<WebStickerTileBundle>(DARK_CAVE_LAYER, METAL)
            .register_ldtk_int_cell_for_layer::<TrapTileBundle>(SPIKES_LAYER, SPIKE_UP)
            .register_ldtk_int_cell_for_layer::<TrapTileBundle>(SPIKES_LAYER, SPIKE_DOWN)
            .register_ldtk_int_cell_for_layer::<TrapTileBundle>(SPIKES_LAYER, SPIKE_LEFT)
            .register_ldtk_int_cell_for_layer::<TrapTileBundle>(SPIKES_LAYER, SPIKE_RIGHT)
            .add_startup_system(spawn::spawn_tilemap)
            .add_system(spawn::spawn_tile_colliders)
            .add_system(spawn::camera_fit_inside_current_level)
            .add_system(spawn::update_level_selection)
            .add_system(change_level);
    }
}

// FOR EASE OF TESTING
pub fn change_level(
    mut level_selection: ResMut<LevelSelection>,
    input: Res<Input<KeyCode>>,
    mut evw_tilemap: EventWriter<TilemapEvent>,
) {
    input.just_pressed(KeyCode::Return).then(|| {
        if let LevelSelection::Identifier(ref id) = *level_selection {
            let index = LEVELS.iter().position(|level| *level == id).unwrap();
            let new_lvl_index = (index + 1) % LEVELS.len();
            *level_selection = LevelSelection::Identifier(String::from(LEVELS[new_lvl_index]));
            evw_tilemap.send(TilemapEvent::ChangedLevel(new_lvl_index));
        }
    });
}

pub fn current_level_index(level_selection: &LevelSelection) -> Option<usize> {
    if let LevelSelection::Identifier(ref id) = *level_selection {
        LEVELS.iter().position(|level| *level == id)
    } else {
        None
    }
}
