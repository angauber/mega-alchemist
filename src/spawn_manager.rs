use crate::game::InGameSystems;
use crate::skeleton::SpawnSkeleton;
use crate::spawn_manager::Enemy::Skeleton;
use bevy::prelude::*;
use bevy_ecs_ldtk::LevelIid;
use std::time::Duration;

pub struct SpawnManagerPlugin;

pub enum Enemy {
    Skeleton,
}

#[derive(Component)]
pub struct SpawnTicker {
    timer: Timer,
    enemy: Enemy,
}

fn setup_spawn_ticker(mut commands: Commands, level_query: Query<Entity, Added<LevelIid>>) {
    let Ok(level_id) = level_query.single() else {
        return;
    };

    commands.entity(level_id).with_child(SpawnTicker {
        timer: Timer::new(Duration::from_millis(1000), TimerMode::Repeating),
        enemy: Skeleton,
    });
}

fn tick_spawners(
    mut commands: Commands,
    mut spawners_query: Query<&mut SpawnTicker>,
    time: Res<Time>,
) {
    for mut spawn_ticker in spawners_query.iter_mut() {
        spawn_ticker.timer.tick(time.delta());

        if spawn_ticker.timer.just_finished() {
            let spawn_event = match spawn_ticker.enemy {
                Skeleton => SpawnSkeleton,
            };

            commands.trigger(spawn_event);
        }
    }
}

impl Plugin for SpawnManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (setup_spawn_ticker, tick_spawners).in_set(InGameSystems),
        );
    }
}
