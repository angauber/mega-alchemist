use crate::transmutation::TransmutationEffect::{Add, Multiply};
use crate::transmutation::{TransmutationEffect, WeaponTransmutation, WeaponTransmutationOffer};
use crate::weapon::Weapon;
use crate::weapon::WeaponStatistic;
use bevy::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

pub struct BuildPlugin;

fn global_multiplier(a: f32, b: f32) -> f32 {
    1. + (a - 1.) + (b - 1.)
}

#[derive(Debug, Copy, Clone)]
pub struct StatisticData {
    base_value: f32,
    modifier: f32,
}

impl StatisticData {
    pub fn from_base_value(base_value: f32) -> Self {
        StatisticData {
            base_value,
            modifier: 1.0,
        }
    }

    pub fn apply_effect(&mut self, effect: &TransmutationEffect) {
        match effect {
            Add(count) => {
                self.base_value += *count as f32;
            }
            Multiply(multiplier) => {
                self.modifier += multiplier;
            }
        }
    }

    pub fn computed_value(&self, player_mod: f32) -> f32 {
        self.base_value * global_multiplier(player_mod, self.modifier)
    }

    pub fn computed_duration(&self, player_mod: f32) -> Duration {
        Duration::from_millis(self.base_value as u64)
            .div_f32(global_multiplier(player_mod, self.modifier))
    }
}

#[derive(Debug)]
pub struct WeaponBuild {
    pub weapon: Weapon,
    pub stats: HashMap<WeaponStatistic, StatisticData>,
}

impl WeaponBuild {
    pub fn statistics(&self) -> Vec<WeaponStatistic> {
        self.stats.keys().cloned().collect()
    }

    pub fn get_stat(&self, stat: &WeaponStatistic) -> Result<&StatisticData, &str> {
        self.stats.get(stat).ok_or("start not part of build")
    }

    pub fn apply_transmutation(&mut self, transmutation: &WeaponTransmutation) -> Result<(), &str> {
        let stat = self
            .stats
            .get_mut(&transmutation.statistic)
            .ok_or("start not part of build")?;

        stat.apply_effect(&transmutation.effect);

        Ok(())
    }
}

#[derive(Resource, Debug)]
pub struct PlayerBuild {
    weapon_builds: HashMap<Weapon, WeaponBuild>,
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
        PlayerBuild::new()
    }
}

impl PlayerBuild {
    pub fn new() -> PlayerBuild {
        PlayerBuild {
            weapon_builds: HashMap::new(),
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

    pub fn add_weapon(&mut self, weapon: Weapon) {
        self.weapon_builds.insert(weapon, weapon.build());
    }

    pub fn apply_weapon_transmutation(
        &mut self,
        offer: &WeaponTransmutationOffer,
    ) -> Result<(), &str> {
        let build = self
            .weapon_builds
            .get_mut(&offer.weapon)
            .ok_or("weapon not found")?;

        for transmutation_option in &offer.transmutations {
            let Some(transmutation) = transmutation_option else {
                continue;
            };

            build.apply_transmutation(transmutation)?;
        }

        Ok(())
    }

    pub fn weapon_builds(&self) -> Vec<&WeaponBuild> {
        self.weapon_builds.values().collect()
    }

    pub fn weapon_build(&self, weapon: &Weapon) -> Result<&WeaponBuild, &str> {
        self.weapon_builds.get(weapon).ok_or("not working")
    }

    pub fn weapon_range(&self, weapon: &Weapon) -> Result<f32, &str> {
        let weapon_range = self
            .weapon_build(weapon)?
            .get_stat(&WeaponStatistic::Range)?;

        Ok(weapon_range.computed_value(self.range))
    }

    pub fn weapon_damage(&self, weapon: &Weapon) -> Result<f32, &str> {
        let weapon_range = self
            .weapon_build(weapon)?
            .get_stat(&WeaponStatistic::Damage)?;

        Ok(weapon_range.computed_value(self.damage))
    }

    pub fn weapon_cooldown(&self, weapon: &Weapon) -> Result<Duration, &str> {
        let weapon_attack_speed = self
            .weapon_build(weapon)?
            .get_stat(&WeaponStatistic::AttackSpeed)?;

        Ok(weapon_attack_speed.computed_duration(self.attack_speed))
    }

    pub fn weapon_projectile_speed(&self, weapon: &Weapon) -> Result<f32, &str> {
        let weapon_projectile_speed = self
            .weapon_build(weapon)?
            .get_stat(&WeaponStatistic::ProjectileSpeed)?;

        Ok(weapon_projectile_speed.computed_value(self.projectile_speed))
    }

    pub fn weapon_projectile_size(&self, weapon: &Weapon) -> Result<f32, &str> {
        let weapon_projectile_size = self
            .weapon_build(weapon)?
            .get_stat(&WeaponStatistic::Size)?;

        Ok(weapon_projectile_size.computed_value(self.projectile_size))
    }

    pub fn weapon_projectile_count(&self, weapon: &Weapon) -> Result<usize, &str> {
        let weapon_projectile_count = self
            .weapon_build(weapon)?
            .get_stat(&WeaponStatistic::ProjectileCount)?
            .base_value as usize;

        Ok(self.projectile_count + weapon_projectile_count)
    }
}

impl Plugin for BuildPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerBuild>();
    }
}
