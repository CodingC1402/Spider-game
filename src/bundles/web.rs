use crate::components::web::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::{Collider, GravityScale, RigidBody, Sensor, Velocity};

#[derive(Bundle, Default)]
pub struct WebStringBundle {
    pub web: WebString,
    pub visual: MaterialMesh2dBundle<ColorMaterial>,
}

#[derive(Bundle, Default)]
pub struct WebHeadBundle {
    pub web_head: WebHead,
    pub sprite: SpriteBundle,
    pub velocity: Velocity,
    pub sensor: Sensor,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub g_scale: GravityScale,
}
