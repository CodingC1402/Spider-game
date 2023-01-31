use bevy::prelude::*;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::prelude::{PhysicsStages, RapierConfiguration};
use bevy_rapier2d::render::RapierDebugRenderPlugin;

mod pre_physics;

#[derive(StageLabel)]
enum PrePhysicsStages {
    ResolveComplexForces,
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(8.0))
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_startup_system(setup_physics)
            .add_stage_before(
                PhysicsStages::SyncBackend,
                PrePhysicsStages::ResolveComplexForces,
                SystemStage::parallel().with_system(pre_physics::compute_complex_external_forces),
            );
    }
}

pub fn setup_physics(mut config: ResMut<RapierConfiguration>) {
    config.gravity = Vec2::new(0.0, -300.0);
}
