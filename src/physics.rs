use bevy::app::*;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PhysicsPlugin;

#[derive(Event)]
pub struct PausePhysics;
#[derive(Event)]
pub struct ResumePhysics;

const PIXELS_PER_METER: f32 = 16.0;

fn rapier_config(mut rapier_config: Query<&mut RapierConfiguration>) {
    let mut config = rapier_config.single_mut().unwrap();

    config.physics_pipeline_active = true;

    config.gravity = Vec2::ZERO;
}

fn on_pause_physics(_event: On<PausePhysics>, mut rapier_config: Query<&mut RapierConfiguration>) {
    let mut config = rapier_config.single_mut().unwrap();
    config.physics_pipeline_active = false;
    info!("Physics simulation paused");
}

fn on_resume_physics(
    _event: On<ResumePhysics>,
    mut rapier_config: Query<&mut RapierConfiguration>,
) {
    let mut config = rapier_config.single_mut().unwrap();
    config.physics_pipeline_active = true;
    info!("Physics simulation resumed");
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER))
            .add_observer(on_pause_physics)
            .add_observer(on_resume_physics)
            .add_systems(Startup, rapier_config);

        #[cfg(feature = "dev_tools")]
        {
            app.add_plugins(RapierDebugRenderPlugin::default());
        }
    }
}
