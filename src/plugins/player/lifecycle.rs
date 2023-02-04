use std::collections::HashMap;

use bevy::prelude::*;
use bevy_ecs_ldtk::LevelSelection;
use bevy_rapier2d::prelude::{
    ActiveEvents, Ccd, CoefficientCombineRule, Collider, ColliderMassProperties, CollisionEvent,
    CollisionGroups, Friction, GravityScale, LockedAxes,
};

use crate::{
    data::{physics::*, player::*, tilemap::Trap},
    plugins::tilemap,
};

use super::PlayerEvent;

const PLAYER_NAME: &str = "Player";
const PLAYER_SIZE: Vec2 = Vec2::splat(24.0);
const PLAYER_SPRITE_SIZE: Vec2 = Vec2::splat(32.0);
const PLAYER_SPRITE_PATH: &str = "spider_sprite_sheet.png";
const PLAYER_SPRITE_ROW_COL: [usize; 2] = [9, 16];
const PLAYER_SPRITE_PADDING: Option<Vec2> = None;
const PLAYER_SPRITE_OFFSET: Option<Vec2> = None;

pub fn spawn_player(
    commands: &mut Commands,
    transform: Transform,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
) -> Entity {
    let texture_handle = asset_server.load(PLAYER_SPRITE_PATH);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        PLAYER_SPRITE_SIZE,
        PLAYER_SPRITE_ROW_COL[0],
        PLAYER_SPRITE_ROW_COL[1],
        PLAYER_SPRITE_PADDING,
        PLAYER_SPRITE_OFFSET,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(PlayerBundle {
            sprite: SpriteSheetBundle {
                transform,
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite {
                    custom_size: Some(PLAYER_SIZE),
                    ..Default::default()
                },
                ..Default::default()
            },
            physics: RigidBodyBundle {
                complex_ef: ComplexExternalForce {
                    forces: HashMap::from([(0, Vec2::ZERO), (1, Vec2::ZERO)]),
                },
                rotation_constraints: LockedAxes::ROTATION_LOCKED_Z,
                gravity_scale: GravityScale(2.5),
                ..Default::default()
            },
            jump: PlayerJump {
                jump_force_id: 1,
                strength: 150.0,
                air_upward_force: 1400.0,
                duration: 0.4,
                ..Default::default()
            },
            movement: PlayerMovement {
                movement_force_id: 0,
                acceleration: 1300.0,
                landing_accel: 200.0,
                airborne_acceleration: PlayerMovement::NORM_AIR_ACCEL,
                max_velocity: 60.0,
                ..Default::default()
            },
            info: PlayerInfo { is_grounded: true },
            name: Name::from(PLAYER_NAME),
            ..Default::default()
        })
        .insert(Ccd::enabled())
        .insert(Name::from("Player"))
        .with_children(|builder| {
            builder
                .spawn(TransformBundle {
                    local: Transform::from_xyz(-3.2, -3.8, 0.0),
                    ..Default::default()
                })
                .insert(ColliderBundle {
                    collider: Collider::capsule_y(0.2, 1.0),
                    friction: Friction {
                        coefficient: 0.0,
                        combine_rule: CoefficientCombineRule::Min,
                    },
                    collision_groups: CollisionGroups {
                        memberships: GameCollisionGroups::PLAYER,
                        filters: GameCollisionGroups::PLAYER.filter_group(),
                    },
                    ..Default::default()
                })
                .insert(Name::from("Padding"));

            builder
                .spawn(TransformBundle {
                    local: Transform::from_xyz(3.2, -3.8, 0.0),
                    ..Default::default()
                })
                .insert(ColliderBundle {
                    collider: Collider::capsule_y(0.2, 1.0),
                    friction: Friction {
                        coefficient: 0.0,
                        combine_rule: CoefficientCombineRule::Min,
                    },
                    collision_groups: CollisionGroups {
                        memberships: GameCollisionGroups::PLAYER,
                        filters: GameCollisionGroups::PLAYER.filter_group(),
                    },
                    ..Default::default()
                })
                .insert(Name::from("Padding"));

            builder
                .spawn(TransformBundle {
                    local: Transform::from_xyz(0.0, -3.3, 0.0),
                    ..Default::default()
                })
                .insert(ColliderBundle {
                    collider: Collider::cuboid(3.5, 2.2),
                    friction: Friction {
                        coefficient: 0.2,
                        combine_rule: CoefficientCombineRule::Min,
                    },
                    collision_groups: CollisionGroups {
                        memberships: GameCollisionGroups::PLAYER,
                        filters: GameCollisionGroups::PLAYER.filter_group(),
                    },
                    active_events: ActiveEvents::COLLISION_EVENTS,
                    ..Default::default()
                })
                .insert(ColliderMassProperties::Mass(1.0))
                .insert(Name::from("Body"));

            builder.spawn(PlayerHeadBundle {
                transform: TransformBundle {
                    local: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..Default::default()
                },
                player_collider: PlayerColliderBundle {
                    collider: Collider::cuboid(1.5, 1.5),
                    ..default()
                },
                name: Name::from("Head"),
                ..Default::default()
            });

            builder.spawn(PlayerFootBundle {
                transform: TransformBundle {
                    local: Transform::from_xyz(0.0, -8.5, 0.0),
                    ..Default::default()
                },
                player_collider: PlayerColliderBundle {
                    collider: Collider::cuboid(1.3, 2.0),
                    ..default()
                },
                name: Name::from("Foot long"),
                ..Default::default()
            });

            builder.spawn(PlayerFootBundle {
                transform: TransformBundle {
                    local: Transform::from_xyz(0.0, -7.0, 0.0),
                    ..Default::default()
                },
                player_collider: PlayerColliderBundle {
                    collider: Collider::cuboid(3.0, 1.0),
                    ..default()
                },
                name: Name::from("Foot wide"),
                ..Default::default()
            });
        })
        .id()
}

pub fn kill_player(
    q_player: Query<Entity, With<Player>>,
    q_trap: Query<Entity, With<Trap>>,
    q_parents: Query<&Parent>,
    mut evr_collisions: EventReader<CollisionEvent>,
    mut evw_death: EventWriter<PlayerEvent>,
) {
    if evr_collisions.is_empty() {
        return;
    }
    let player = q_player.single();
    let player_is_parent = |entity: &Entity| {
        if let Ok(parent) = q_parents.get(*entity) {
            return parent.get() == player;
        } else {
            false
        }
    };
    for collision in evr_collisions.iter() {
        if let CollisionEvent::Started(entity_one, entity_two, _) = collision {
            let other_entity = player_is_parent(entity_one)
                .then_some(*entity_two)
                .or_else(|| player_is_parent(entity_one).then_some(*entity_one));
            if let None = other_entity {
                continue;
            }
            let other_entity = other_entity.unwrap();
            q_trap.get(other_entity).is_ok().then(|| {
                evw_death.send(PlayerEvent::Died(player));
                return;
            });
        }
    }
}

pub fn respawn_player_on_death(
    mut q_player: Query<&mut Transform, With<Player>>,
    level_selection: Res<LevelSelection>,
    mut evr_death: EventReader<PlayerEvent>,
) {
    evr_death.iter().find(|ev| match ev {
        PlayerEvent::Died(_) => {
            respawn_player(&mut q_player, &*level_selection);
            true
        }
        _ => false,
    });
}

fn respawn_player(
    q_player: &mut Query<&mut Transform, With<Player>>,
    level_selection: &LevelSelection,
) {
    match tilemap::current_level_index(level_selection) {
        Some(level_index) => {
            let mut player = q_player.single_mut();
            player.translation = translation_in_level(level_index);
        }
        _ => (),
    };
}

pub fn translation_in_level(level_index: usize) -> Vec3 {
    match level_index {
        0 => Vec3::new(44.0, 340.0, 900.0),
        1 => Vec3::new(340.0, 332.0, 900.0),
        2 => Vec3::new(544.0, 368.0, 900.0),
        3 => Vec3::new(696.0, 240.0, 900.0),
        4 => Vec3::new(832.0, 380.0, 900.0),
        5 => Vec3::new(1300.0, 450.0, 900.0),
        _ => Vec3::ZERO,
    }
}

pub fn player_collision(
    q_player: Query<Entity, With<Player>>,
    q_parents: Query<&Parent>,
    mut evr_collisions: EventReader<CollisionEvent>,
    mut evw_player: EventWriter<PlayerEvent>,
) {
    if evr_collisions.is_empty() {
        return;
    }
    let player = q_player.single();
    let player_is_parent = |entity: &Entity| {
        if let Ok(parent) = q_parents.get(*entity) {
            return parent.get() == player;
        } else {
            false
        }
    };
    for collision in evr_collisions.iter() {
        if let CollisionEvent::Started(entity_one, entity_two, _) = collision {
            let other_entity = player_is_parent(entity_one)
                .then_some(*entity_two)
                .or_else(|| player_is_parent(entity_one).then_some(*entity_one));
            if let None = other_entity {
                continue;
            }
            let other_entity = other_entity.unwrap();
            evw_player.send(PlayerEvent::Collided(other_entity));
        }
    }
}
