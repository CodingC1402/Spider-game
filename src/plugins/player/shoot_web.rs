use crate::bundles::web::*;
use crate::components::{
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
const WEB_Z: f32 = 800.0;

#[derive(Resource, Default)]
pub struct WebTexture(Handle<Image>);

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
            if !q_web.is_empty() {
                let entity = q_web.single();
                commands.entity(entity).despawn_recursive();
            }

            let player_translation = q_player.single().translation().truncate();
            let shoot_direction = (cursor_translation - player_translation).normalize();
            let angle = shoot_direction.angle_between(Vec2::X);

            let mat_handle = if q_web_string.is_empty() {
                materials.add(ColorMaterial::from(Color::WHITE)).clone()
            } else {
                q_web_string.single().clone()
            };
            let mesh_handle = meshes.add(shape::Box::new(1.0, 1.0, 1.0).into());

            commands
                .spawn(Web)
                .insert(SpatialBundle {
                    transform: Transform::from_translation(player_translation.extend(0.0)),
                    ..default()
                })
                .with_children(|child_builder| {
                    child_builder.spawn(WebHeadBundle {
                        web_head: WebHead::default(),
                        sprite: SpriteBundle {
                            texture: web_texture.0.clone(),
                            sprite: Sprite {
                                custom_size: Some(8.0 * Vec2::ONE),
                                ..default()
                            },
                            transform: Transform::from_translation(
                                4.0 * shoot_direction.extend(WEB_Z + 0.1),
                            ),
                            ..default()
                        },
                        collider: Collider::ball(0.1),
                        velocity: Velocity::linear(WEB_SHOOT_SPEED * shoot_direction),
                        g_scale: GravityScale(0.0),
                        ..default()
                    });
                    child_builder.spawn(WebStringBundle {
                        visual: MaterialMesh2dBundle {
                            mesh: mesh_handle.into(),
                            material: mat_handle,
                            transform: Transform::from_rotation(Quat::from_rotation_z(-angle))
                                .with_scale(Vec3::new(0.0, 0.2, 1.0)),
                            ..default()
                        },
                        ..default()
                    });
                });
        }
    }
}

pub fn update_web_string_transform(
    mut q_web_string: Query<&mut Transform, With<WebString>>,
    q_web_head: Query<&Transform, (With<WebHead>, Without<WebString>)>,
) {
    if q_web_head.is_empty() || q_web_string.is_empty() {
        return;
    }
    let web_head_translation = q_web_head.single().translation.truncate();
    let mut web_string = q_web_string.single_mut();

    let midpoint = web_head_translation / 2.0;
    let angle = web_head_translation.angle_between(Vec2::X);
    web_string.scale = Vec3::new(web_head_translation.length(), 0.2, 1.0);
    web_string.translation = midpoint.extend(WEB_Z);
    web_string.rotation = Quat::from_rotation_z(-angle);
}
