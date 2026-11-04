use crate::game::GameState;
use bevy::app::*;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::geometry::CollisionEventFlags;

pub struct PhysicsPlugin;

const PIXELS_PER_METER: f32 = 16.0;

pub fn started_collision_pairs(
    mut events: MessageReader<CollisionEvent>,
    filter_flags: CollisionEventFlags,
) -> Vec<(Entity, Entity)> {
    events
        .read()
        .filter_map(|event| match event {
            CollisionEvent::Started(a, b, f) if filter_flags.contains(*f) => Some((a, b)),
            _ => None,
        })
        .flat_map(|(&a, &b)| [(a, b), (b, a)])
        .collect()
}

fn rapier_config(mut rapier_config: Query<&mut RapierConfiguration>) {
    let mut config = rapier_config.single_mut().unwrap();

    config.physics_pipeline_active = true;

    config.gravity = Vec2::ZERO;
}

fn pause_physics(mut rapier_config: Query<&mut RapierConfiguration>) {
    let Ok(mut config) = rapier_config.single_mut() else {
        return;
    };

    config.physics_pipeline_active = false;
    info!("Physics simulation paused");
}

fn resume_physics(mut rapier_config: Query<&mut RapierConfiguration>) {
    let Ok(mut config) = rapier_config.single_mut() else {
        return;
    };

    config.physics_pipeline_active = true;
    info!("Physics simulation resumed");
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .add_systems(OnEnter(GameState::InGame), resume_physics)
        .add_systems(OnEnter(GameState::Paused), pause_physics)
        .add_systems(Startup, rapier_config);

        #[cfg(feature = "dev_tools")]
        {
            app.add_plugins(RapierDebugRenderPlugin::default());
        }
    }
}
