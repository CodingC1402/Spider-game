use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::data::physics::ComplexExternalForce;

pub fn compute_complex_external_forces(
    mut q_cef: Query<(&mut ExternalForce, &ComplexExternalForce)>,
) {
    q_cef.for_each_mut(|(mut ef, cef)| {
        ef.force = cef.composite_force();
        // warn!("ef force: {}", ef.force);
    });
}
