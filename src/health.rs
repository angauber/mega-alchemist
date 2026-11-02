use crate::game::{GameOver, InGameSystems};
use crate::player::Player;
use bevy::prelude::*;

pub struct HealthPlugin;

#[derive(Component, Default, Debug)]
pub struct Health {
    pub point: f32,
}

pub fn entities_health_system(
    mut commands: Commands,
    health_query: Query<(Entity, &Health)>,
    player_query: Query<Entity, With<Player>>,
) {
    for (entity, health) in &health_query {
        if health.point <= 0. {
            if player_query.contains(entity) {
                commands.trigger(GameOver);
            }

            commands.entity(entity).despawn();
        }
    }
}

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, entities_health_system.in_set(InGameSystems));
    }
}
