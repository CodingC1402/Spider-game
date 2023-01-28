use crate::components::web::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, CollisionGroups, GravityScale, RigidBody, Sensor, Velocity,
};

#[derive(Bundle, Default)]
pub struct WebStringBundle {
    pub web_string: WebString,
    pub web: Web,
    pub visual: MaterialMesh2dBundle<ColorMaterial>,
}

#[derive(Bundle, Default)]
pub struct WebHeadBundle {
    pub web_head: WebHead,
    pub web: Web,
    pub sprite: SpriteBundle,
    pub velocity: Velocity,
    pub sensor: Sensor,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub g_scale: GravityScale,
    pub collision_groups: CollisionGroups,
    pub active_events: ActiveEvents,
}
