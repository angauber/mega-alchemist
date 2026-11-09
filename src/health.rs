use crate::build::PlayerBuild;
use crate::game::{GameOver, InGameSystems};
use crate::level::IncreasePlayerXP;
use crate::player::Player;
use bevy::prelude::*;

pub struct HealthPlugin;

#[derive(Component, Default, Debug)]
pub struct Health {
    pub max: f32,
    pub points: f32,
}

impl Health {
    pub fn new(points: f32) -> Self {
        Health {
            max: points,
            points,
        }
    }
}

pub fn entities_health_system(
    mut commands: Commands,
    health_query: Query<(Entity, &Health)>,
    player_query: Query<Entity, With<Player>>,
) {
    for (entity, health) in &health_query {
        if health.points < 1. {
            if player_query.contains(entity) {
                commands.trigger(GameOver);
                return;
            }

            commands.trigger(IncreasePlayerXP(health.max as usize));

            commands.entity(entity).despawn();
        }
    }
}

pub fn regen_health_system(
    mut player_query: Query<&mut Health, With<Player>>,
    player_build: Res<PlayerBuild>,
    time: Res<Time>,
) {
    let Ok(mut health) = player_query.single_mut() else {
        return;
    };

    health.points += player_build.regen * time.delta_secs() * 2.0;
    health.points = health.points.clamp(0.0, health.max);
}

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, entities_health_system.in_set(InGameSystems))
            .add_systems(Update, regen_health_system.in_set(InGameSystems));
    }
}
