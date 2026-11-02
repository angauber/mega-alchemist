use crate::game::InGameSystems;
use crate::skeleton::SpawnSkeleton;
use crate::spawn_manager::Enemy::{Bat, Skeleton};
use bevy::prelude::*;
use bevy_ecs_ldtk::LevelIid;
use rand::distr::weighted::WeightedIndex;
use rand::prelude::*;
use std::time::Duration;

pub struct SpawnManagerPlugin;

// Bank total value
#[derive(Resource, Default)]
pub struct BankValue(f32);

// Budget per second
#[derive(Resource)]
pub struct BankBudget(f32);

impl Default for BankBudget {
    fn default() -> Self {
        BankBudget(10.0)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Enemy {
    Bat,
    Skeleton,
}

impl Enemy {
    pub fn cost(&self) -> f32 {
        match self {
            Bat => 10.0,
            Skeleton => 100.0,
        }
    }

    pub fn spawn_event<'a>(&self) -> impl Event<Trigger<'a>: Default> {
        SpawnSkeleton
    }
}

#[derive(Resource)]
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

impl SpawnWeights {
    pub fn random_enemy(&self, bank_value: f32) -> Option<Enemy> {
        let affordable_enemies = Self::affordable_enemies(bank_value);

        if affordable_enemies.is_empty() {
            return None;
        }

        let weights: Vec<usize> = affordable_enemies
            .iter()
            .map(|enemy| self.weight(enemy))
            .collect();

        let weighted_index = WeightedIndex::new(&weights).unwrap();
        let mut rng = rand::rng();
        let index = weighted_index.sample(&mut rng);

        Some(affordable_enemies[index])
    }

    fn weight(&self, enemy: &Enemy) -> usize {
        match enemy {
            Bat => self.bat,
            Skeleton => self.skeleton,
        }
    }

    fn affordable_enemies(bank_value: f32) -> Vec<Enemy> {
        [Bat, Skeleton]
            .into_iter()
            .filter(|enemy| enemy.cost() <= bank_value)
            .collect()
    }
}

#[derive(Component)]
pub struct SpawnTicker {
    timer: Timer,
}

fn setup_manager(mut commands: Commands, level_query: Query<Entity, Added<LevelIid>>) {
    let Ok(level_id) = level_query.single() else {
        return;
    };

    commands.insert_resource(BankValue::default());
    commands.insert_resource(BankValue::default());
    commands.insert_resource(BankBudget::default());
    commands.insert_resource(SpawnWeights::default());

    commands.entity(level_id).with_child(SpawnTicker {
        timer: Timer::new(Duration::from_millis(50), TimerMode::Repeating),
    });
}

fn accumulate_bank_value(
    mut bank_value: ResMut<BankValue>,
    bank_budget: Res<BankBudget>,
    time: Res<Time>,
) {
    bank_value.0 += bank_budget.0 * time.delta_secs();
}

fn increase_bank_budget_over_time(mut bank_budget: ResMut<BankBudget>, time: Res<Time>) {
    bank_budget.0 += time.delta_secs() * 0.1;
}

fn spawner(
    mut commands: Commands,
    spawn_weights: Res<SpawnWeights>,
    mut bank_value: ResMut<BankValue>,
    mut spawn_ticker_query: Query<&mut SpawnTicker>,
    time: Res<Time>,
) {
    let Ok(mut spawn_ticker) = spawn_ticker_query.single_mut() else {
        return;
    };

    spawn_ticker.timer.tick(time.delta());

    if !spawn_ticker.timer.is_finished() {
        return;
    }

    // randomly pick an affordable enemy to spawn
    let Some(enemy) = spawn_weights.random_enemy(bank_value.0) else {
        return;
    };

    info!("Spawning enemy {:?}", enemy);

    commands.trigger(enemy.spawn_event());

    bank_value.0 -= enemy.cost();
}

impl Plugin for SpawnManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BankValue>()
            .init_resource::<BankBudget>()
            .init_resource::<SpawnWeights>()
            .add_systems(
                Update,
                (
                    setup_manager,
                    accumulate_bank_value,
                    increase_bank_budget_over_time,
                    spawner,
                )
                    .in_set(InGameSystems),
            );
    }
}
