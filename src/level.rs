use crate::game::{GameOverWithDelay, GameState, InGameSystems};
use crate::transmutation::TransmutationEffect::Multiply;
use crate::transmutation::TransmutationStatistic::{
    AttackSpeed, Damage, ProjectileCount, ProjectileSpeed,
};
use crate::transmutation::{Transmutation, WeaponTransmutationOffer};
use crate::ui::transmutation::DisplayTransmutationOffers;
use crate::weapon::WeaponType;
use bevy::prelude::*;
use std::time::Duration;

pub struct LevelPlugin;

#[derive(Resource, Default)]
pub struct PlayerLevel(usize);

#[derive(Resource)]
struct LevelTimer(Timer);

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
    mut next_state: ResMut<NextState<GameState>>,
) {
    timer.0.tick(time.delta());

    if !timer.0.is_finished() {
        return;
    }

    next_state.set(GameState::Paused);

    let a = WeaponTransmutationOffer {
        identifier: 0,
        weapon: WeaponType::Knife,
        transmutations: [
            Some(Transmutation {
                statistic: Damage,
                effect: Multiply(0.05),
            }),
            None,
            None,
        ],
    };

    let b = WeaponTransmutationOffer {
        identifier: 1,
        weapon: WeaponType::Knife,
        transmutations: [
            Some(Transmutation {
                statistic: AttackSpeed,
                effect: Multiply(0.08),
            }),
            None,
            None,
        ],
    };

    let c = WeaponTransmutationOffer {
        identifier: 2,
        weapon: WeaponType::Knife,
        transmutations: [
            Some(Transmutation {
                statistic: ProjectileSpeed,
                effect: Multiply(0.07),
            }),
            Some(Transmutation {
                statistic: ProjectileCount,
                effect: crate::transmutation::TransmutationEffect::Add(1),
            }),
            None,
        ],
    };

    player_level.0 += 1;

    commands.trigger(DisplayTransmutationOffers {
        offer_a: a.into(),
        offer_b: b.into(),
        offer_c: c.into(),
    })
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
