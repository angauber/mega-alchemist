use crate::bat::SpawnBat;
use crate::game::InGameSystems;
use crate::player::Player;
use crate::skeleton::SpawnSkeleton;
use crate::spawn_manager::Enemy::{Bat, Skeleton};
use crate::spawner::Spawner;
use bevy::prelude::*;
use bevy_ecs_ldtk::LevelIid;
use rand::distr::weighted::WeightedIndex;
use rand::prelude::*;
use std::time::Duration;

pub struct SpawnManagerPlugin;

#[derive(Copy, Clone, Debug)]
pub enum Enemy {
    Bat,
    Skeleton,
}

impl Enemy {
    pub fn all() -> Vec<Self> {
        vec![Bat, Skeleton]
    }

    pub fn cost(&self) -> f32 {
        match self {
            Bat => 10.0,
            Skeleton => 25.0,
        }
    }
}

pub struct SpawnWeights {
    bat: usize,
    skeleton: usize,
}

impl Default for SpawnWeights {
    fn default() -> Self {
        SpawnWeights {
            bat: 10,
            skeleton: 1,
        }
    }
}

#[derive(Resource, Default)]
pub struct SpawnTicker(Timer);

#[derive(Resource)]
pub struct SpawnManager {
    weights: SpawnWeights,
    memory: Option<Enemy>,
    bank_value: f32,
    bank_budget: f32,
}

impl Default for SpawnManager {
    fn default() -> Self {
        SpawnManager {
            weights: SpawnWeights::default(),
            memory: None,
            bank_value: 0.0,
            bank_budget: 10.0,
        }
    }
}

impl SpawnManager {
    pub fn bank_value(&self) -> f32 {
        self.bank_value
    }

    pub fn accumulate_bank_value(&mut self, dt: f32) {
        self.bank_value += self.bank_budget * dt;
    }

    pub fn increase_bank_budget(&mut self, dt: f32) {
        self.bank_budget += dt * 0.001
    }

    pub fn spawnable_enemy(&mut self) -> Option<Enemy> {
        let random_enemy = self.memory.unwrap_or_else(|| self.random_enemy());

        if self.memory.is_none() {
            self.memory = Some(random_enemy);
        }

        if random_enemy.cost() > self.bank_value {
            return None;
        };

        Some(random_enemy)
    }

    pub fn spawned(&mut self) {
        let Some(spawned) = self.memory else {
            warn!("no spawned memory available");
            return;
        };

        self.bank_value -= spawned.cost();
        self.memory = None;
    }

    fn random_enemy(&mut self) -> Enemy {
        let enemies = Enemy::all();

        let weights: Vec<usize> = enemies.iter().map(|enemy| self.weight(enemy)).collect();

        let weighted_index = WeightedIndex::new(&weights).unwrap();
        let mut rng = rand::rng();
        let index = weighted_index.sample(&mut rng);

        enemies[index]
    }

    fn weight(&self, enemy: &Enemy) -> usize {
        match enemy {
            Bat => self.weights.bat,
            Skeleton => self.weights.skeleton,
        }
    }
}

fn setup_manager(mut commands: Commands, level_query: Query<Entity, Added<LevelIid>>) {
    let Ok(level_id) = level_query.single() else {
        return;
    };

    info!("Setting up manager for level {}", level_id);
    commands.insert_resource(SpawnManager::default());

    let timer = Timer::new(Duration::from_millis(50), TimerMode::Repeating);

    commands.entity(level_id).with_child(SpawnTicker(timer));
}

fn accumulate_bank_value(mut spawn_manager: ResMut<SpawnManager>, time: Res<Time>) {
    spawn_manager.accumulate_bank_value(time.delta_secs());
}

fn increase_bank_budget_over_time(mut spawn_manager: ResMut<SpawnManager>, time: Res<Time>) {
    spawn_manager.increase_bank_budget(time.delta_secs());
}

fn spawn_enemy(
    mut commands: Commands,
    mut spawn_manager: ResMut<SpawnManager>,
    mut spawn_ticker: ResMut<SpawnTicker>,
    spawners_query: Query<&Transform, With<Spawner>>,
    player_position_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    spawn_ticker.0.tick(time.delta());

    if !spawn_ticker.0.is_finished() {
        return;
    }

    let Ok(player_transform) = player_position_query.single() else {
        return;
    };

    let Some(enemy_to_spawn) = spawn_manager.spawnable_enemy() else {
        return;
    };

    // randomly pick the spawn
    let player_position = player_transform.translation.xy();

    let spawner_pool = spawners_query
        .iter()
        .filter(|spawner_position| {
            spawner_position.translation.xy().distance(player_position) >= 300.
        })
        .take(5)
        .collect::<Vec<_>>();

    let Some(chosen_spawner) = spawner_pool.choose(&mut rand::rng()) else {
        warn!("no Spawner found");
        return;
    };

    let mut enemy_position = **chosen_spawner;
    enemy_position.translation.z = 5.0;

    info!(
        "Spawning enemy {:?} | bank value: {} | enemy cost: {}",
        enemy_to_spawn,
        spawn_manager.bank_value(),
        enemy_to_spawn.cost()
    );

    match enemy_to_spawn {
        Bat => {
            commands.trigger(SpawnBat(enemy_position));
        }
        Skeleton => {
            commands.trigger(SpawnSkeleton(enemy_position));
        }
    }

    spawn_manager.spawned();
}

impl Plugin for SpawnManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawnTicker>()
            .init_resource::<SpawnManager>()
            .add_systems(
                Update,
                (
                    setup_manager,
                    accumulate_bank_value,
                    increase_bank_budget_over_time,
                    spawn_enemy,
                )
                    .in_set(InGameSystems),
            );
    }
}
