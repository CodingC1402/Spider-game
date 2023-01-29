use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody, Velocity, GravityScale, Friction, ColliderMassProperties, LockedAxes, Sleeping, Ccd, ExternalForce, ExternalImpulse};

#[derive(Clone, Debug, Default, Bundle)]
pub struct RigidBodyBundle {
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub gravity_scale: GravityScale,
    pub rotation_constraints: LockedAxes,
    pub force: ExternalForce,
    pub impulse: ExternalImpulse,

}

#[derive(Clone, Debug, Default, Bundle)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub density: ColliderMassProperties,
    pub friction: Friction,
}