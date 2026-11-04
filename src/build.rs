use bevy::prelude::*;

pub struct BuildPlugin;

#[derive(Resource, Debug)]
pub struct PlayerBuild {
    pub max_health: f32,
    pub move_speed: f32,
    pub damage: f32,
    pub range: f32,
    pub attack_speed: f32,
    pub projectile_count: usize,
    pub projectile_size: f32,
    pub projectile_speed: f32,
}

impl Default for PlayerBuild {
    fn default() -> Self {
        PlayerBuild {
            max_health: 100.0,
            move_speed: 1.0,
            damage: 1.0,
            range: 1.0,
            attack_speed: 1.0,
            projectile_count: 0,
            projectile_size: 1.0,
            projectile_speed: 1.0,
        }
    }
}

impl Plugin for BuildPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerBuild>();
    }
}
