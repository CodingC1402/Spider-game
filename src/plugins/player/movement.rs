use bevy::prelude::*;
use bevy_rapier2d::prelude::{ExternalForce, ExternalImpulse, Velocity};

use crate::components::player::{Player, PlayerInfo, PlayerJump, PlayerMovement};

use super::{PlayerControl, PlayerEvent};

pub fn handle_movement(
    input: Res<Input<KeyCode>>,
    control: Res<PlayerControl>,
    mut query: Query<
        (
            &mut PlayerMovement,
            &PlayerInfo,
            &mut Velocity,
            &mut ExternalForce,
        ),
        With<Player>,
    >,
) {
    let (mut movement, info, mut vel, mut force) = query.single_mut();
    let old_value = movement.axis;

    movement.axis = input.pressed(control.left).then_some(-1.0).unwrap_or(0.0)
        + input.pressed(control.right).then_some(1.0).unwrap_or(0.0);

    // Reduce vel when change direction or stop moving but still
    // still keep the possibility of speed boost.
    let vel_vec = old_value * movement.axis;
    let linvel = vel.linvel.clone();
    let reduce_vel = || {
        vel.linvel = Vec2::new(0.0, vel.linvel.y);
    };
    let mut apply_force_to_player = || {
        force.force = Vec2::new(
            (linvel.x.abs() < movement.max_velocity)
                .then_some(
                    movement.axis
                        * info
                            .is_grounded
                            .then_some(movement.acceleration)
                            .unwrap_or(movement.airborne_acceleration),
                )
                .unwrap_or(0.0),
            force.force.y,
        )
    };

    (vel_vec <= 0.0).then(reduce_vel);
    apply_force_to_player();
}

pub fn apply_accel_when_land(
    mut e_reader: EventReader<PlayerEvent>,
    mut query: Query<(&PlayerMovement, &mut ExternalImpulse), With<Player>>,
) {
    e_reader.iter().for_each(|e| {
        if let PlayerEvent::Grounded(player) = e {
            let (movement, mut force) = query
                .get_mut(*player)
                .expect("Player entity in event is not qualified as a player");
            
            force.impulse += Vec2::new(movement.landing_accel * movement.axis, 0.0);
        }
    });
}

pub fn send_vel_based_event(mut e_writer: EventWriter<PlayerEvent>) {}
