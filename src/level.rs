use crate::game::{GameOverWithDelay, InGameSystems};
use crate::transmutation::DrawTransmutationOffers;
use bevy::prelude::*;
use std::time::Duration;

pub struct LevelPlugin;

#[derive(Resource)]
pub struct PlayerLevel(pub usize);

#[derive(Resource)]
struct LevelTimer(Timer);

impl Default for PlayerLevel {
    fn default() -> Self {
        PlayerLevel(1)
    }
}

impl Default for LevelTimer {
    fn default() -> Self {
        LevelTimer(Timer::new(Duration::from_secs(30), TimerMode::Repeating))
    }
}

fn tick_level_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<LevelTimer>,
    mut player_level: ResMut<PlayerLevel>,
) {
    timer.0.tick(time.delta());

    if !timer.0.is_finished() {
        return;
    }

    player_level.0 += 1;

    commands.trigger(DrawTransmutationOffers);
}

fn clear_level_resources(
    _event: On<GameOverWithDelay>,
    mut player_level: ResMut<PlayerLevel>,
    mut level_timer: ResMut<LevelTimer>,
) {
    *player_level = PlayerLevel::default();
    *level_timer = LevelTimer::default();
}
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(clear_level_resources)
            .init_resource::<PlayerLevel>()
            .init_resource::<LevelTimer>()
            .add_systems(Update, tick_level_timer.in_set(InGameSystems));
    }
}
