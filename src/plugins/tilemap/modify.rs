use bevy::prelude::*;
use bevy::text::Text2dBounds;

use crate::data::player::Player;
use crate::data::tilemap::*;
use crate::plugins::player::PlayerEvent;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{current_level_index, FontHandle, LevelChanged};

const ASPECT_RATIO: f32 = 16. / 9.;
const TEXT_COLOR: Color = Color::rgb(0.95, 0.95, 0.95);

#[derive(Resource)]
pub struct CreditTimer {
    pub timer: Timer,
    pub line_count: u8,
    pub active: bool,
}

pub fn camera_fit_inside_current_level(
    mut camera_query: Query<(
        &mut bevy::render::camera::OrthographicProjection,
        &mut Transform,
    )>,
    player_query: Query<&GlobalTransform, With<Player>>,
    level_query: Query<(&Transform, &Handle<LdtkLevel>), Without<OrthographicProjection>>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
    level_selection: Res<LevelSelection>,
) {
    let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();
    let player_translation = player_query.single().translation();

    for (level_transform, level_handle) in &level_query {
        if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
            let level = &ldtk_level.level;
            if level_selection.is_match(&0, level) {
                let level_ratio = level.px_wid as f32 / ldtk_level.level.px_hei as f32;

                orthographic_projection.scaling_mode = bevy::render::camera::ScalingMode::None;
                orthographic_projection.bottom = 0.;
                orthographic_projection.left = 0.;
                if level_ratio > ASPECT_RATIO {
                    // level is wider than the screen
                    orthographic_projection.top = (level.px_hei as f32 / 9.).round() * 9.;
                    orthographic_projection.right = orthographic_projection.top * ASPECT_RATIO;
                    camera_transform.translation.x = (player_translation.x
                        - level_transform.translation.x
                        - orthographic_projection.right / 2.)
                        .clamp(0., level.px_wid as f32 - orthographic_projection.right);
                    camera_transform.translation.y = 0.;
                } else {
                    // level is taller than the screen
                    orthographic_projection.right = (level.px_wid as f32 / 16.).round() * 16.;
                    orthographic_projection.top = orthographic_projection.right / ASPECT_RATIO;
                    camera_transform.translation.y = (player_translation.y
                        - level_transform.translation.y
                        - orthographic_projection.top / 2.)
                        .clamp(0., level.px_hei as f32 - orthographic_projection.top);
                    camera_transform.translation.x = 0.;
                }

                camera_transform.translation.x += level_transform.translation.x;
                camera_transform.translation.y += level_transform.translation.y;
            }
        }
    }
}

pub fn update_level_selection(
    level_query: Query<(&Handle<LdtkLevel>, &Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    mut level_selection: ResMut<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
    mut evw_level_changed: EventWriter<LevelChanged>,
) {
    let previous_level = current_level_index(&*level_selection);
    for (level_handle, level_transform) in &level_query {
        if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
            let level_bounds = Rect {
                min: Vec2::new(level_transform.translation.x, level_transform.translation.y),
                max: Vec2::new(
                    level_transform.translation.x + ldtk_level.level.px_wid as f32,
                    level_transform.translation.y + ldtk_level.level.px_hei as f32,
                ),
            };

            for player_transform in &player_query {
                if player_transform.translation.x < level_bounds.max.x
                    && player_transform.translation.x > level_bounds.min.x
                    && player_transform.translation.y < level_bounds.max.y
                    && player_transform.translation.y > level_bounds.min.y
                    && !level_selection.is_match(&0, &ldtk_level.level)
                {
                    *level_selection =
                        LevelSelection::Identifier(ldtk_level.level.identifier.clone());
                    if let Some(level) = current_level_index(&*level_selection) {
                        if let Some(previous) = previous_level {
                            evw_level_changed.send(LevelChanged {
                                previous,
                                current: level,
                            });
                        }
                    }
                }
            }
        }
    }
}

pub fn spawn_surface_edges(
    q_esen: Query<Entity, With<EdgeSensor>>,
    q_edge: Query<Entity, (With<Edge>, Without<Collider>)>,
    mut evr_player_collisions: EventReader<PlayerEvent>,
    mut commands: Commands,
) {
    if q_esen.is_empty() {
        return;
    }
    evr_player_collisions.iter().find(|ev| {
        if let Some(entity) = if_collide_event(ev) {
            q_esen
                .contains(entity)
                .then(|| {
                    q_edge.for_each(|entity| {
                        commands.entity(entity).insert(Collider::cuboid(
                            super::TILE_HALF_SIZE.0,
                            super::TILE_HALF_SIZE.1,
                        ));
                    });
                })
                .is_some()
        } else {
            false
        }
    });
}

pub fn spawn_credits(
    q_ssen: Query<Entity, With<CreditsSensor>>,
    q_cred: Query<(Entity, &Transform), With<Credits>>,
    font_handle: Res<FontHandle>,
    mut cred_timer: ResMut<CreditTimer>,
    mut evr_player_collisions: EventReader<PlayerEvent>,
    mut commands: Commands,
) {
    if q_ssen.is_empty() {
        return;
    }
    evr_player_collisions.iter().find(|ev| {
        if let Some(entity) = if_collide_event(ev) {
            q_ssen
                .contains(entity)
                .then(|| {
                    let (cred_entity, cred_transform) = q_cred.single();
                    commands.entity(cred_entity).insert(Text2dBundle {
                        text: Text::from_section(
                            "SPIDER ESCAPE",
                            TextStyle {
                                font: font_handle.0.clone(),
                                color: TEXT_COLOR,
                                font_size: 25.0,
                                ..default()
                            },
                        )
                        .with_alignment(TextAlignment::CENTER_RIGHT),
                        text_2d_bounds: Text2dBounds {
                            size: Vec2::new(300.0, 500.0),
                        },
                        transform: Transform::from_translation(cred_transform.translation)
                            .with_scale(0.75 * Vec3::ONE),
                        ..default()
                    });
                    cred_timer.active = true;
                })
                .is_some()
        } else {
            false
        }
    });
}

pub fn update_credit_timer(
    q_cred: Query<&GlobalTransform, With<Credits>>,
    font_handle: Res<FontHandle>,
    mut cred_timer: ResMut<CreditTimer>,
    time: Res<Time>,
    mut commands: Commands,
) {
    cred_timer.active.then(|| {
        cred_timer
            .timer
            .tick(time.delta())
            .just_finished()
            .then(|| {
                let credits_translation = q_cred.single().translation();
                match cred_timer.line_count {
                    0 => {
                        commands.spawn(Text2dBundle {
                            text: Text::from_section(
                                "A game by Long Do and Nguyen Pham",
                                TextStyle {
                                    font: font_handle.0.clone(),
                                    font_size: 14.0,
                                    color: TEXT_COLOR,
                                },
                            )
                            .with_alignment(TextAlignment::CENTER_RIGHT),
                            transform: Transform::from_translation(
                                credits_translation - 15.0 * Vec3::Y,
                            )
                            .with_scale(0.75 * Vec3::ONE),
                            ..default()
                        });
                        cred_timer.line_count += 1;
                    }
                    1 => {
                        commands.spawn(Text2dBundle {
                            text: Text::from_section(
                                "Thanks for playing!",
                                TextStyle {
                                    font: font_handle.0.clone(),
                                    font_size: 14.0,
                                    color: TEXT_COLOR,
                                },
                            )
                            .with_alignment(TextAlignment::CENTER_RIGHT),
                            transform: Transform::from_translation(
                                credits_translation - 30.0 * Vec3::Y,
                            )
                            .with_scale(0.75 * Vec3::ONE),
                            ..default()
                        });
                        cred_timer.line_count += 1;
                    }
                    2 => cred_timer.active = false,
                    _ => (),
                };
            });
    });
}

pub fn collect_coin(
    q_coin: Query<(Entity, &GlobalTransform), With<Coin>>,
    font_handle: Res<FontHandle>,
    mut evr_player_collisions: EventReader<PlayerEvent>,
    mut commands: Commands,
) {
    if q_coin.is_empty() {
        return;
    }
    let (coin_entity, coin_transform) = q_coin.single();
    evr_player_collisions
        .iter()
        .find(|ev| {
            if let Some(entity) = if_collide_event(ev) {
                (entity == coin_entity).then(|| {
                    commands.spawn(Text2dBundle {
                        text: Text::from_section(
                            "You have acquired the Coin of Luck.\nMay your journey be safe and bring you to what you seek.",
                            TextStyle {
                                font: font_handle.0.clone(),
                                font_size: 20.0,
                                ..default()
                            },
                        )
                        .with_alignment(TextAlignment::CENTER_RIGHT),
                        text_2d_bounds: Text2dBounds {
                            size: Vec2::new(350.0, 150.0),
                        },
                        transform: Transform::from_translation(coin_transform.translation() + 12.0 * Vec3::Y).with_scale(0.2 * Vec3::ONE),
                        ..default()
                    });
                    commands.entity(coin_entity).despawn_recursive();
                }).is_some()
            } else { false }
        });
}

fn if_collide_event(ev: &&PlayerEvent) -> Option<Entity> {
    match ev {
        PlayerEvent::Collided(entity) => Some(*entity),
        _ => None,
    }
}
