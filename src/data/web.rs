use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

#[derive(Component, Default, Reflect)]
pub struct WebString;

#[derive(Component, Default, Reflect)]
pub struct WebHead;

#[derive(Component, Default, Reflect)]
pub struct Web {
    pub attached: bool,
    pub pull_force_id: Option<u8>,
    pub initial_web_length: Option<f32>,
}

#[derive(Bundle, Default)]
pub struct WebStringBundle {
    pub web_string: WebString,
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
    pub collision_groups: CollisionGroups,
    pub active_events: ActiveEvents,
}
