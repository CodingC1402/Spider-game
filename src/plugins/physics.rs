use bevy::prelude::*;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::prelude::RapierConfiguration;
use bevy_rapier2d::render::RapierDebugRenderPlugin;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0))
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_startup_system(setup_physic);
    }
}

fn setup_physic(mut config: ResMut<RapierConfiguration>) {
    config.gravity = Vec2::new(0.0, -360.0);
}