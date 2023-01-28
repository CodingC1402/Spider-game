use bevy::prelude::*;
use bevy_rapier2d::prelude::{ExternalForce, ExternalImpulse};

use crate::components::player::{PlayerJump, PlayerInfo, Player, PlayerMovement};

use super::{PlayerControl, PlayerEvent};

pub fn handle_movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    control: Res<PlayerControl>,
    mut e_writer: EventWriter<PlayerEvent>,
    mut query: Query<
        (
            Entity,
            &mut PlayerMovement,
            &PlayerInfo,
            &mut ExternalForce,
            &mut ExternalImpulse,
        ),
        With<Player>,
    >,
) {
    
}