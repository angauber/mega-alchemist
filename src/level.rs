use crate::game::GameOverWithDelay;
use crate::transmutation::DrawTransmutationOffers;
use bevy::math::ops::floor;
use bevy::prelude::*;

pub struct LevelPlugin;

const XP_BASE: f32 = 100.;
const POW_MOD: f32 = 2.0;

#[derive(Event)]
pub struct IncreasePlayerXP(pub usize);

#[derive(Resource, Default)]
pub struct PlayerLevel {
    xp: usize,
}

impl PlayerLevel {
    pub fn xp(&self) -> usize {
        self.xp
    }

    pub fn increase_xp(&mut self, points: usize) -> usize {
        let current_level = self.level();

        self.xp += points;

        let new_level = self.level();

        info!(
            "current level: {}  | new level: {}",
            current_level, new_level
        );

        new_level - current_level
    }

    pub fn level(&self) -> usize {
        1 + floor((self.xp as f32 / XP_BASE).powf(1. / POW_MOD)) as usize
    }
}

fn clear_level_resources(_event: On<GameOverWithDelay>, mut player_level: ResMut<PlayerLevel>) {
    *player_level = PlayerLevel::default();
}

fn increase_player_xp(
    event: On<IncreasePlayerXP>,
    mut player_level: ResMut<PlayerLevel>,
    mut commands: Commands,
) {
    if player_level.increase_xp(event.0) > 0 {
        commands.trigger(DrawTransmutationOffers);
    }
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(increase_player_xp)
            .add_observer(clear_level_resources)
            .init_resource::<PlayerLevel>();
    }
}
