use std::ops::Not;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::player::{Player, PlayerInfo, PlayerJump};

use super::{PlayerControl, PlayerEvent};

pub fn handle_jump(
    input: Res<Input<KeyCode>>,
    control: Res<PlayerControl>,
    mut e_writer: EventWriter<PlayerEvent>,
    mut query: Query<
        (
            Entity,
            &PlayerJump,
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
            .for_each(|(entity, jump_com, info_com, mut force, mut impulse)| {
                if !info_com.is_grounded {
                    return
                }

                impulse.impulse = Vec2::new(0.0, jump_com.strength);
                e_writer.send(PlayerEvent::Jumped(entity));
            });
    });
}

pub fn handle_landing(
    mut e_reader: EventReader<PlayerEvent>,
    query: Query<&mut PlayerInfo, With<Player>>,
) {
    e_reader.iter().all(|e| true);
}
