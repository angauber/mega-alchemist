use crate::game::InGameSystems;
use crate::player::Player;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct AgentPlugin;

#[derive(Component, Default, Debug)]
pub struct Enemy;

#[derive(Component, Default, Debug)]
pub struct AgentSteering {
    pub max_speed: f32,
    pub max_accel: f32,
}

pub fn agent_steering_system(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut agent_query: Query<(&mut Velocity, &Transform, &AgentSteering), Without<Player>>,
) {
    let Ok(player_position) = player_query.single() else {
        return;
    };

    for (mut agent_velocity, agent_position, agent_steering) in agent_query.iter_mut() {
        let dt = time.delta_secs();

        let dir = player_position.translation.xy() - agent_position.translation.xy();

        let dir = dir.normalize_or_zero();

        let target = dir * agent_steering.max_speed;
        let rate = agent_steering.max_accel;

        agent_velocity.linear = agent_velocity.linear.move_towards(target, rate * dt);
    }
}

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, agent_steering_system.in_set(InGameSystems));
    }
}
