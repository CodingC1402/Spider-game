use bevy::prelude::*;
use bevy_rapier2d::prelude::{ExternalForce, ExternalImpulse};

use crate::components::player::{Player, PlayerInfo, PlayerJump, PlayerMovement};

use super::{PlayerControl, PlayerEvent};

pub fn handle_movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    control: Res<PlayerControl>,
    mut e_writer: EventWriter<PlayerEvent>,
    mut query: Query<(Entity, &mut PlayerMovement, &PlayerInfo, &mut ExternalForce), With<Player>>,
) {
    let (player, mut movement, info, mut externalforce) = query.single_mut();
    let old_value = movement.axis;

    movement.axis = input.pressed(control.left).then(|| -1.0).unwrap_or(0.0)
        + input.pressed(control.right).then(|| 1.0).unwrap_or(0.0);

    let vel_vec = old_value * movement.axis;
    (vel_vec <= 0.0).then(|| {
        
    });
}

pub fn send_vel_based_event(mut e_writer: EventWriter<PlayerEvent>) {

}