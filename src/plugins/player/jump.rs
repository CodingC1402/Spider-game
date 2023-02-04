use std::ops::Not;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::data::{
    physics::ComplexExternalForce,
    player::{Player, PlayerFoot, PlayerHead, PlayerInfo, PlayerJump},
    tilemap::Platform,
};

use super::{PlayerControl, PlayerEvent};

pub fn handle_jump(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    control: Res<PlayerControl>,
    mut e_writer: EventWriter<PlayerEvent>,
    mut query: Query<
        (
            Entity,
            &mut PlayerJump,
            &PlayerInfo,
            &mut ComplexExternalForce,
            &mut ExternalImpulse,
        ),
        With<Player>,
    >,
) {
    let pressing = input.pressed(control.jump);
    let just_pressed = input.just_pressed(control.jump);

    query.iter_mut().for_each(
        |(entity, mut jump_com, info_com, mut cef, mut impulse)| {
            (just_pressed && info_com.is_grounded).then(|| {
                impulse.impulse = Vec2::new(impulse.impulse.x, jump_com.strength);
                jump_com.counter = jump_com.duration;

                e_writer.send(PlayerEvent::Jumped(entity));
            });

            (jump_com.counter > 0.0).then(|| {
                pressing
                    .then(|| {
                        cef.forces
                            .entry(jump_com.jump_force_id)
                            .and_modify(|jump_force| {
                                jump_force.y = jump_com.air_upward_force
                                    * (jump_com.counter / jump_com.duration)
                            });
                        jump_com.counter -= time.delta_seconds();
                    })
                    .unwrap_or_else(|| {
                        cef.forces
                            .entry(jump_com.jump_force_id)
                            .and_modify(|jump_force| {
                                jump_force.y = 0.0
                            });
                        jump_com.counter = 0.0;
                    });
            });
        },
    );
}

pub fn check_if_head_bump(
    q_child: Query<&Children>,
    q_head: Query<Entity, (With<PlayerHead>, With<Sensor>)>,
    q_platform: Query<&Collider, With<Platform>>,
    mut q_player: Query<(Entity, &mut PlayerJump, &mut ComplexExternalForce), With<Player>>,
    rapier_context: Res<RapierContext>,
) {
    let (player, mut player_jump, mut cef) = q_player.single_mut();

    let check_head_then =
        |child: Entity, func: &dyn Fn() -> bool| {
            q_head.contains(child).then(func).unwrap_or(false)
        };

    let check_not_collide = |child: Entity| {
        !check_head_then(child, &|| {
            rapier_context
                .intersections_with(child)
                .any(|(entity1, entity2, _)| {
                    q_platform.contains(entity1) || q_platform.contains(entity2)
                })
        })
    };

    q_child
        .iter_descendants(player)
        .all(check_not_collide)
        .not()
        .then(|| {
            player_jump.counter = 0.0;
            cef.forces.entry(player_jump.jump_force_id).and_modify(|jump_force| {
                jump_force.y = 0.0;
            });
        });
}

pub fn check_if_grounded(
    q_child: Query<&Children>,
    q_foot: Query<Entity, (With<PlayerFoot>, With<Sensor>)>,
    q_platform: Query<&Collider, With<Platform>>,
    mut q_player: Query<(Entity, &mut PlayerInfo), With<Player>>,
    rapier_context: Res<RapierContext>,
    mut e_writer: EventWriter<PlayerEvent>,
) {
    let (player, mut player_info) = q_player.single_mut();

    let check_foot_then =
        |child: Entity, func: &dyn Fn() -> bool| {
            q_foot.contains(child).then(func).unwrap_or(false)
        };

    let check_not_collide = |child: Entity| {
        !check_foot_then(child, &|| {
            rapier_context
                .intersections_with(child)
                .any(|(entity1, entity2, _)| {
                    q_platform.contains(entity1) || q_platform.contains(entity2)
                })
        })
    };

    let old_value = player_info.is_grounded;
    player_info.is_grounded = q_child
        .iter_descendants(player)
        .all(check_not_collide)
        .not();

    (old_value != player_info.is_grounded).then(|| {
        e_writer.send(
            player_info
                .is_grounded
                .then_some(PlayerEvent::Grounded(player))
                .unwrap_or(PlayerEvent::Airborne(player)),
        )
    });
}
