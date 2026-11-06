use crate::bat::SpawnBat;
use crate::damage::PlayerDamage;
use crate::game::{GameClock, GameOver, InGameSystems};
use crate::player::{Player, PlayerPressure};
use crate::skeleton::SpawnSkeleton;
use crate::spawn_manager::Enemy::{Bat, Skeleton};
use crate::spawner::Spawner;
use bevy::prelude::*;
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
            Bat => 15.0,
            Skeleton => 50.0,
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

#[derive(Resource)]
pub struct SpawnTicker(Timer);

impl Default for SpawnTicker {
    fn default() -> Self {
        SpawnTicker(Timer::new(Duration::from_millis(10), TimerMode::Repeating))
    }
}

#[derive(Resource)]
pub struct SpawnManager {
    weights: SpawnWeights,
    memory: Option<Enemy>,
    bank_value: f32,
    bank_budget: f32,
    target_pressure_seconds: f32,
    catch_up_time: f32,
}

impl Default for SpawnManager {
    fn default() -> Self {
        SpawnManager {
            weights: SpawnWeights::default(),
            memory: None,
            bank_value: 0.0,
            bank_budget: 0.0,
            target_pressure_seconds: 7.0,
            catch_up_time: 5.0,
        }
    }
}

impl SpawnManager {
    pub fn bank_value(&self) -> f32 {
        self.bank_value
    }

    #[cfg(feature = "dev_tools")]
    pub fn bank_budget(&self) -> f32 {
        self.bank_budget
    }

    pub fn accumulate_bank_value(&mut self, dt: f32) {
        self.bank_value += self.bank_budget * dt;
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

    pub fn update_bank_budget(&mut self, in_game_time: f32, player_dps: f32, player_pressure: f32) {
        let initial_min_budget = 5.0;
        let time_based_budget = in_game_time * 0.04;

        let min_budget = time_based_budget.max(initial_min_budget);
        let max_budget = 1_000.;

        // e.g. n seconds of player damage
        let target_pressure = self.target_pressure_seconds * player_dps;
        let new_budget = player_dps + ((target_pressure - player_pressure) / self.catch_up_time);

        self.bank_budget = new_budget.clamp(min_budget, max_budget);
    }
}

fn update_bank_budget(
    mut spawn_manager: ResMut<SpawnManager>,
    player_damage: Res<PlayerDamage>,
    player_pressure: Res<PlayerPressure>,
    game_clock: Res<GameClock>,
) {
    let play_time = game_clock.0.elapsed_secs();
    let player_dps = player_damage.per_second;

    spawn_manager.update_bank_budget(play_time, player_dps, player_pressure.0);
}

fn accumulate_bank_value(mut spawn_manager: ResMut<SpawnManager>, time: Res<Time>) {
    spawn_manager.accumulate_bank_value(time.delta_secs());
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
        .collect::<Vec<_>>();

    let Some(chosen_spawner) = spawner_pool.choose(&mut rand::rng()) else {
        warn!("no Spawner found");
        return;
    };

    let enemy_position = **chosen_spawner;

    info!(
        "Spawning enemy {:?} at {} | bank value: {} enemy cost: {}",
        enemy_to_spawn,
        enemy_position.translation,
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

pub fn on_game_over(_event: On<GameOver>, mut spawn_manager: ResMut<SpawnManager>) {
    info!("spawn manager reset");
    *spawn_manager = SpawnManager::default();
}

impl Plugin for SpawnManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawnTicker>()
            .init_resource::<SpawnManager>()
            .add_observer(on_game_over)
            .add_systems(
                Update,
                (accumulate_bank_value, update_bank_budget, spawn_enemy).in_set(InGameSystems),
            );
    }
}
