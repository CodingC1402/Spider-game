use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, LockedAxes, RigidBody, GravityScale};

use crate::{bundles::{physics::{ColliderBundle, RigidBodyBundle}, player::PlayerBundle}, components::player::{PlayerJump, PlayerMovement, PlayerInfo}};

const PLAYER_NAME: &str = "Player";
const PLAYER_SIZE: Vec2 = Vec2::splat(32.0);
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
                gravity_scale: GravityScale(4.0),
                ..Default::default()
            },
            jump: PlayerJump {
                strength: 800.0,
                ..Default::default()
            },
            movement: PlayerMovement {
                ..Default::default()
            },
            info: PlayerInfo {
                is_grounded: true
            },
            name: Name::from(PLAYER_NAME),
            ..Default::default()
        })
        .with_children(|builder| {
            builder
                .spawn(TransformBundle::default())
                .insert(ColliderBundle {
                    collider: Collider::cuboid(16.0, 10.0),
                    ..Default::default()
                });
        })
        .id()
}