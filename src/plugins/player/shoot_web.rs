use crate::data::physics::ComplexExternalForce;
use crate::data::tilemap::WebStickable;
use crate::data::web::*;
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
// const WEB_MAX_STRETCH: f32 = 100.0;
const WEB_Z: f32 = 100.0;
const WEB_PULL_FORCE_SCALE: f32 = 80.0;

#[derive(Resource, Default)]
pub struct WebTexture(Handle<Image>);

pub struct DespawnWebEvent;

pub fn setup_web_texture(asset_server: Res<AssetServer>, mut web_texture: ResMut<WebTexture>) {
    web_texture.0 = asset_server.load(WEB_SPRITE_PATH);
}

pub fn handle_shoot_web_input(
    input: Res<Input<MouseButton>>,
    mut evw_web: EventWriter<PlayerEvent>,
    mut evw_despawn_web: EventWriter<DespawnWebEvent>,
) {
    if input.just_pressed(MouseButton::Left) {
        evw_web.send(PlayerEvent::ShotWeb);
        debug!("!shot web");
    } else if input.just_pressed(MouseButton::Right) {
        evw_despawn_web.send(DespawnWebEvent);
        debug!("!released web");
    }
}

/// Shoot the web-head out with a certain velocity, the "string" of the web starts out with 0 length, which will
/// later be adjusted by [`update_web_string_transform`].
pub fn shoot_web(
    q_web_string: Query<&Handle<ColorMaterial>, With<WebString>>,
    q_web: Query<Entity, With<Web>>,
    q_player: Query<&GlobalTransform, With<Player>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    web_texture: Res<WebTexture>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
    mut evr_player_action: EventReader<PlayerEvent>,
    mut commands: Commands,
) {
    evr_player_action
        .iter()
        .any(|ev| match ev {
            PlayerEvent::ShotWeb => true,
            _ => false,
        })
        .then(|| {
            if !q_web.is_empty() {
                return;
            }
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
                let (midpoint, angle) =
                    midpoint_and_angle_to_x(player_translation, shoot_translation);

                let mat_handle = q_web_string
                    .is_empty()
                    .then(|| materials.add(ColorMaterial::from(Color::WHITE)).clone())
                    .unwrap_or_else(|| q_web_string.single().clone());
                let mesh_handle = meshes.add(shape::Box::new(1.0, 1.0, 1.0).into());

                commands
                    .spawn(SpatialBundle::default())
                    .insert(Web::default())
                    .with_children(|child_builder| {
                        child_builder
                            .spawn(WebHeadBundle {
                                web_head: WebHead::default(),
                                sprite: SpriteBundle {
                                    texture: web_texture.0.clone(),
                                    sprite: Sprite {
                                        custom_size: Some(8.0 * Vec2::ONE),
                                        ..default()
                                    },
                                    transform: Transform::from_translation(
                                        shoot_translation.extend(WEB_Z + 1.0),
                                    ),
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
                            })
                            .insert(Name::from("Web head"));
                        child_builder
                            .spawn(WebStringBundle {
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
                            })
                            .insert(Name::from("Web string"));
                    });
            }
        });
}

pub fn update_web_string_and_pull_force(
    q_web: Query<&Web>,
    mut q_web_string: Query<&mut Transform, With<WebString>>,
    q_web_head: Query<&GlobalTransform, (With<WebHead>, Without<WebString>)>,
    mut q_player: Query<(&GlobalTransform, &mut ComplexExternalForce, &mut Damping), With<Player>>,
) {
    if q_web_head.is_empty() || q_web_string.is_empty() {
        return;
    }
    let web_head_translation = q_web_head.single().translation().truncate();
    let (player_transform, mut cef, mut damping) = q_player.single_mut();
    let player_translation = player_transform.translation().truncate();
    let player_to_web_head = web_head_translation - player_translation;
    let length = player_to_web_head.length();

    let web = q_web.single();
    let mut web_string = q_web_string.single_mut();
    let (midpoint, angle) = midpoint_and_angle_to_x(player_translation, web_head_translation);
    web_string.scale = Vec3::new(length, 0.2, 1.0);
    web_string.translation = midpoint.extend(WEB_Z);
    web_string.rotation = Quat::from_rotation_z(-angle);
    web.attached.then(|| {
        cef.forces
            .entry(web.pull_force_id.unwrap())
            .and_modify(|pull_dir| {
                let initial_web_length = web.initial_web_length.unwrap();
                *pull_dir = WEB_PULL_FORCE_SCALE
                    * (player_to_web_head.length() - initial_web_length).clamp(0.0, f32::MAX)
                    * player_to_web_head.normalize();
                (pull_dir.length() > 0.0).then(|| {
                    damping.linear_damping = 0.5;
                });
            });
    });
}

pub fn handle_web_head_collision(
    mut q_web_head: Query<(Entity, &mut Velocity, &GlobalTransform), With<WebHead>>,
    mut q_web: Query<(Entity, &mut Web)>,
    q_web_stickable: Query<Entity, With<WebStickable>>,
    q_player_transform: Query<&GlobalTransform, With<Player>>,
    mut q_cef: Query<&mut ComplexExternalForce, With<Player>>,
    mut evr_collisions: EventReader<CollisionEvent>,
    mut commands: Commands,
) {
    if evr_collisions.is_empty() || q_web_head.is_empty() {
        return;
    }
    let (web_head, mut web_head_vel, web_head_transform) = q_web_head.single_mut();
    let player_transform = q_player_transform.single();
    for collision in evr_collisions.iter() {
        if let CollisionEvent::Started(entity_one, entity_two, _) = collision {
            let other_entity = (web_head == *entity_one)
                .then(|| *entity_two)
                .unwrap_or_else(|| *entity_one);

            q_web_stickable
                .get(other_entity)
                .is_ok()
                .then(|| {
                    web_head_vel.linvel = Vec2::ZERO;
                    // web swinging starts
                    //
                    let mut cef = q_cef.single_mut();
                    let next_force_id = cef.next_force();
                    let pull_dir = web_head_transform.translation().truncate()
                        - player_transform.translation().truncate();
                    cef.forces.insert(next_force_id, pull_dir.normalize());

                    let (_, mut web) = q_web.single_mut();
                    web.attached = true;
                    web.pull_force_id = Some(next_force_id);
                    web.initial_web_length = Some(pull_dir.length());
                })
                .unwrap_or_else(|| {
                    q_web.for_each(|(entity, _)| {
                        commands.entity(entity).despawn_recursive();
                    });
                });
        }
    }
}

pub fn despawn_web(
    q_web: Query<(Entity, &Web)>,
    mut q_player_cef: Query<&mut ComplexExternalForce, With<Player>>,
    evr_despawn_web: EventReader<DespawnWebEvent>,
    mut commands: Commands,
) {
    if evr_despawn_web.is_empty() || q_web.is_empty() {
        return;
    }
    evr_despawn_web.clear();
    let (web_entity, web) = q_web.single();
    let mut cef = q_player_cef.single_mut();
    if let Some(pull_force_id) = web.pull_force_id {
        cef.forces.remove(&pull_force_id);
    }
    commands.entity(web_entity).despawn_recursive();
}

fn midpoint_and_angle_to_x(start: Vec2, end: Vec2) -> (Vec2, f32) {
    let midpoint = (start + end) / 2.0;
    let angle = (end - start).angle_between(Vec2::X);
    (midpoint, angle)
}
