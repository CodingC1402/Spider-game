use crate::data::web::*;
use crate::data::tilemap::{NonStickable, WebStickable};
use crate::data::{
    physics::{CollisionGroupsFilter, GameCollisionGroups},
    player::Player,
    web::{Web, WebHead, WebString},
};
use crate::plugins::utils;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

use super::PlayerEvent;

const WEB_SPRITE_PATH: &str = "web.png";
const WEB_SHOOT_SPEED: f32 = 150.0;
const WEB_MAX_STRETCH: f32 = 100.0;
const WEB_Z: f32 = 100.0;

#[derive(Resource, Default)]
pub struct WebTexture(Handle<Image>);

pub struct WebOverstretchedEvent;

pub fn setup_web_texture(asset_server: Res<AssetServer>, mut web_texture: ResMut<WebTexture>) {
    web_texture.0 = asset_server.load(WEB_SPRITE_PATH);
}

pub fn handle_shoot_web_input(
    input: Res<Input<MouseButton>>,
    mut ev_writer: EventWriter<PlayerEvent>,
) {
    if input.just_pressed(MouseButton::Left) {
        ev_writer.send(PlayerEvent::ShotWeb);
        debug!("shot web");
    }
}

/// Shoot the web-head out with a certain velocity, the "string" of the web starts out with 0 length, which will
/// later be adjusted by [`update_web_string_transform`].
///
/// Also responsible for despawning the last web shot, if any.
pub fn shoot_web(
    q_web_string: Query<&Handle<ColorMaterial>, With<WebString>>,
    q_web: Query<Entity, With<Web>>,
    q_player: Query<&GlobalTransform, With<Player>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    web_texture: Res<WebTexture>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
    mut ev_reader: EventReader<PlayerEvent>,
    mut commands: Commands,
) {
    if ev_reader
        .iter()
        .filter(|ev| match ev {
            PlayerEvent::ShotWeb => true,
            _ => false,
        })
        .count()
        > 0
    {
        let (camera, camera_transform) = q_camera.single();
        if let Some(cursor_translation) =
            utils::cursor_screen_to_world(&windows, &camera, &camera_transform)
        {
            q_web.for_each(|entity| {
                commands.entity(entity).despawn_recursive();
            });

            let player_translation = q_player.single().translation().truncate();
            let shoot_direction = (cursor_translation - player_translation).normalize();
            let shoot_translation = player_translation + 4.0 * shoot_direction;
            let (midpoint, angle) = midpoint_and_angle_to_x(player_translation, shoot_translation);

            let mat_handle = if q_web_string.is_empty() {
                materials.add(ColorMaterial::from(Color::WHITE)).clone()
            } else {
                q_web_string.single().clone()
            };
            let mesh_handle = meshes.add(shape::Box::new(1.0, 1.0, 1.0).into());

            commands.spawn(WebHeadBundle {
                web_head: WebHead::default(),
                sprite: SpriteBundle {
                    texture: web_texture.0.clone(),
                    sprite: Sprite {
                        custom_size: Some(8.0 * Vec2::ONE),
                        ..default()
                    },
                    transform: Transform::from_translation(shoot_translation.extend(WEB_Z + 1.0)),
                    ..default()
                },
                collider: Collider::ball(0.1),
                velocity: Velocity::linear(WEB_SHOOT_SPEED * shoot_direction),
                g_scale: GravityScale(0.0),
                collision_groups: CollisionGroups {
                    memberships: GameCollisionGroups::WEB,
                    filters: GameCollisionGroups::WEB.filter_group(),
                },
                active_events: ActiveEvents::COLLISION_EVENTS,
                ..default()
            });
            commands.spawn(WebStringBundle {
                visual: MaterialMesh2dBundle {
                    mesh: mesh_handle.into(),
                    material: mat_handle,
                    transform: Transform {
                        translation: midpoint.extend(WEB_Z),
                        rotation: Quat::from_rotation_z(-angle),
                        scale: Vec3::new(0.0, 0.2, 1.0),
                    },
                    ..default()
                },
                ..default()
            });
        }
    }
}

pub fn update_web_string_transform(
    mut q_web_string: Query<&mut Transform, With<WebString>>,
    q_web_head: Query<&GlobalTransform, (With<WebHead>, Without<WebString>)>,
    q_player: Query<&GlobalTransform, With<Player>>,
    mut ev_overstretch: EventWriter<WebOverstretchedEvent>,
) {
    if q_web_head.is_empty() || q_web_string.is_empty() {
        return;
    }
    let web_head_translation = q_web_head.single().translation().truncate();
    let player_translation = q_player.single().translation().truncate();
    let length = (web_head_translation - player_translation).length();
    if length > WEB_MAX_STRETCH {
        ev_overstretch.send(WebOverstretchedEvent);
        return;
    }

    let mut web_string = q_web_string.single_mut();
    let (midpoint, angle) = midpoint_and_angle_to_x(player_translation, web_head_translation);
    web_string.scale = Vec3::new(length, 0.2, 1.0);
    web_string.translation = midpoint.extend(WEB_Z);
    web_string.rotation = Quat::from_rotation_z(-angle);
}

pub fn despawn_overstretched_web(
    q_web: Query<Entity, With<Web>>,
    ev_overstretch: EventReader<WebOverstretchedEvent>,
    mut commands: Commands,
) {
    if ev_overstretch.is_empty() {
        return;
    }
    ev_overstretch.clear();
    q_web.for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
}

pub fn handle_web_head_collision(
    mut q_web_head: Query<(Entity, &mut Velocity), With<WebHead>>,
    q_web: Query<Entity, With<Web>>,
    q_non_stick: Query<Entity, With<NonStickable>>,
    q_web_stickable: Query<Entity, With<WebStickable>>,
    mut ev_collisions: EventReader<CollisionEvent>,
    mut commands: Commands,
) {
    if ev_collisions.is_empty() || q_web_head.is_empty() {
        return;
    }
    let (web_head, mut web_head_vel) = q_web_head.single_mut();
    for collision in ev_collisions.iter() {
        if let CollisionEvent::Started(entity_one, entity_two, _) = collision {
            let other_entity = if web_head == *entity_one {
                *entity_two
            } else {
                *entity_one
            };
            if let Ok(_) = q_web_stickable.get(other_entity) {
                web_head_vel.linvel = Vec2::ZERO;
            } else if let Ok(_) = q_non_stick.get(other_entity) {
                q_web.for_each(|entity| {
                    commands.entity(entity).despawn_recursive();
                });
            }
        }
    }
}

fn midpoint_and_angle_to_x(start: Vec2, end: Vec2) -> (Vec2, f32) {
    let midpoint = (start + end) / 2.0;
    let angle = (end - start).angle_between(Vec2::X);
    (midpoint, angle)
}
