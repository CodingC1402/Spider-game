use std::ops::Not;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::{
    player::{Player, PlayerFoot, PlayerInfo, PlayerJump},
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
            &mut ExternalForce,
            &mut ExternalImpulse,
        ),
        With<Player>,
    >,
) {
    input.just_pressed(control.jump).then(|| {
        query
            .iter_mut()
            .for_each(|(entity, mut jump_com, info_com, mut force, mut impulse)| {
                if !info_com.is_grounded {
                    return;
                }

                impulse.impulse = Vec2::new(0.0, jump_com.strength);
                jump_com.counter = jump_com.duration;

                e_writer.send(PlayerEvent::Jumped(entity));
            });
    });
}

pub fn check_if_grounded(
    q_child: Query<&Children>,
    q_foot: Query<Entity, (With<PlayerFoot>, With<Sensor>)>,
    mut q_player: Query<(Entity, &mut PlayerInfo), With<Player>>,
    q_platform: Query<&Collider, With<Platform>>,
    rapier_context: Res<RapierContext>,
) {
    let (player, mut player_info) = q_player.single_mut();

    let check_foot_then = |child: Entity, func: &dyn Fn() -> bool| {
        q_foot.contains(child).then(func).unwrap_or(false)
    };

    let check_not_collide = |child: Entity| {
        !check_foot_then(child, & || {
            rapier_context
                .intersections_with(child)
                .any(|(entity1, entity2, _)| {
                    q_platform.contains(entity1) || q_platform.contains(entity2)
                })
        })
    };

    #[cfg(debug_assertions)]
    let old_value = player_info.is_grounded;

    player_info.is_grounded = q_child
        .iter_descendants(player)
        .all(check_not_collide)
        .not();

    #[cfg(debug_assertions)]
    (old_value != player_info.is_grounded)
        .then(|| info!("Player grounded = {}", player_info.is_grounded));
}
