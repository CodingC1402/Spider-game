use bevy::prelude::*;
use bevy_rapier2d::prelude::{
    Ccd, CoefficientCombineRule, Collider, Friction, GravityScale, LockedAxes,
};

use crate::data::{physics::*, player::*};

const PLAYER_NAME: &str = "Player";
const PLAYER_SIZE: Vec2 = Vec2::splat(16.0);
const PLAYER_SPRITE_SIZE: Vec2 = Vec2::splat(512.0);
const PLAYER_SPRITE_PATH: &str = "spider.png";
const PLAYER_SPRITE_ROW_COL: [usize; 2] = [1, 1];
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
            physic: RigidBodyBundle {
                rotation_constraints: LockedAxes::ROTATION_LOCKED_Z,
                gravity_scale: GravityScale(2.5),
                ..Default::default()
            },
            jump: PlayerJump {
                strength: 250.0,
                air_upward_force: 1800.0,
                duration: 0.5,
                ..Default::default()
            },
            movement: PlayerMovement {
                acceleration: 1500.0,
                landing_accel: 100.0,
                airborne_acceleration: 600.0,
                max_velocity: 60.0,
                ..Default::default()
            },
            info: PlayerInfo { is_grounded: true },
            name: Name::from(PLAYER_NAME),
            ..Default::default()
        })
        .insert(Ccd::enabled())
        .with_children(|builder| {
            builder
                .spawn(TransformBundle::default())
                .insert(ColliderBundle {
                    collider: Collider::capsule_x(2.0, 3.5),
                    friction: Friction {
                        coefficient: 0.4,
                        combine_rule: CoefficientCombineRule::Min,
                    },
                    ..Default::default()
                })
                .insert(Name::from("Body"));

            builder.spawn(PlayerHeadBundle {
                transform: TransformBundle {
                    local: Transform::from_xyz(0.0, 4.0, 0.0),
                    ..Default::default()
                },
                collider: Collider::cuboid(3.0, 2.0),
                name: Name::from("Head"),
                ..Default::default()
            });

            builder.spawn(PlayerFootBundle {
                transform: TransformBundle {
                    local: Transform::from_xyz(0.0, -5.0, 0.0),
                    ..Default::default()
                },
                collider: Collider::cuboid(2.0, 4.0),
                name: Name::from("Foot long"),
                ..Default::default()
            });

            builder.spawn(PlayerFootBundle {
                transform: TransformBundle {
                    local: Transform::from_xyz(0.0, -4.0, 0.0),
                    ..Default::default()
                },
                collider: Collider::cuboid(6.0, 1.0),
                name: Name::from("Foot wide"),
                ..Default::default()
            });
        })
        .id()
}
