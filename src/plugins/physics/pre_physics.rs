use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::data::physics::ComplexExternalForce;

pub fn compute_complex_external_forces(
    mut q_cef: Query<(&mut ExternalForce, &ComplexExternalForce)>,
) {
    q_cef.for_each_mut(|(mut ef, cef)| {
        let mut final_force = Vec2::ZERO;
        cef.forces.iter().for_each(|(_, force)| {
            final_force += *force;
        });
        ef.force = final_force;
    });
}
