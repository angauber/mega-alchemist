use crate::manuscript::Manuscript;
use crate::transmutation::TransmutationEffect::{Add, Multiply};
use crate::transmutation::{Transmutation, TransmutationEffect, WeaponTransmutationOffer};
use crate::weapon::Weapon::{Bow, Knife};
use crate::weapon::{Weapon, bow_build, knife_build};
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use std::time::Duration;

#[allow(dead_code)]
const MAX_WEAPONS: usize = 2;
const MAX_MANUSCRIPTS: usize = 3;

pub struct BuildPlugin;

fn global_multiplier(a: f32, b: f32) -> f32 {
    1. + (a - 1.) + (b - 1.)
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Statistic {
    Range,
    Damage,
    MoveSpeed,
    ProjectileSize,
    AttackSpeed,
    ProjectileSpeed,
    ProjectileCount,
    Bounces,
    Piercing,
}

impl Statistic {
    pub fn is_incremental(&self) -> bool {
        matches!(
            self,
            Statistic::ProjectileCount | Statistic::Bounces | Statistic::Piercing
        )
    }
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

#[derive(Clone, Debug)]
pub struct WeaponBuild {
    level: usize,
    stats: HashMap<Statistic, StatisticData>,
}

impl From<Weapon> for WeaponBuild {
    fn from(value: Weapon) -> Self {
        WeaponBuild {
            level: 0,
            stats: match value {
                Knife => knife_build(),
                Bow => bow_build(),
            },
        }
    }
}

impl WeaponBuild {
    pub fn statistics(&self) -> Vec<Statistic> {
        self.stats.keys().cloned().collect()
    }

    pub fn get_stat(&self, stat: &Statistic) -> Result<&StatisticData, &str> {
        self.stats.get(stat).ok_or("start not part of build")
    }

    pub fn apply_transmutation(&mut self, transmutation: &Transmutation) {
        let missing_stat_error =
            format!("stat {:?} not part of WeaponBuild", transmutation.statistic);

        let stat = self
            .stats
            .get_mut(&transmutation.statistic)
            .expect(missing_stat_error.as_str());

        stat.apply_effect(&transmutation.effect);
    }
}

fn format_percentage(multiplier: f32) -> String {
    format!("{:.0}%", (multiplier - 1.0) * 100.)
}

#[derive(Resource, Clone, Debug)]
pub struct PlayerBuild {
    weapon_builds: HashMap<Weapon, WeaponBuild>,
    manuscripts: HashMap<Manuscript, usize>,
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
            manuscripts: HashMap::new(),
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
        self.weapon_builds.insert(weapon, weapon.into());
    }

    pub fn add_manuscript(&mut self, manuscript: &Manuscript) {
        self.manuscripts.insert(*manuscript, 1);

        self.apply_manuscript_to_build(manuscript);
    }

    pub fn apply_weapon_transmutation(&mut self, offer: &WeaponTransmutationOffer) {
        let build = self
            .weapon_builds
            .get_mut(&offer.weapon)
            .expect("weapon not found");

        build.level += 1;

        for transmutation_option in &offer.transmutations {
            let Some(transmutation) = transmutation_option else {
                continue;
            };

            build.apply_transmutation(transmutation);
        }
    }

    pub fn apply_manuscript_transmutation(&mut self, manuscript: &Manuscript) {
        self.manuscripts
            .insert(*manuscript, self.manuscripts[manuscript] + 1);

        self.apply_manuscript_to_build(manuscript);
    }

    pub fn weapon_builds(&self) -> Vec<(&Weapon, &WeaponBuild)> {
        self.weapon_builds.iter().collect()
    }

    pub fn weapon_build(&self, weapon: &Weapon) -> Result<&WeaponBuild, &str> {
        self.weapon_builds.get(weapon).ok_or("not working")
    }

    pub fn weapon_range(&self, weapon: &Weapon) -> Result<f32, &str> {
        let weapon_range = self.weapon_build(weapon)?.get_stat(&Statistic::Range)?;

        Ok(weapon_range.computed_value(self.range))
    }

    pub fn weapon_damage(&self, weapon: &Weapon) -> Result<f32, &str> {
        let weapon_range = self.weapon_build(weapon)?.get_stat(&Statistic::Damage)?;

        Ok(weapon_range.computed_value(self.damage))
    }

    pub fn weapon_cooldown(&self, weapon: &Weapon) -> Result<Duration, &str> {
        let weapon_attack_speed = self
            .weapon_build(weapon)?
            .get_stat(&Statistic::AttackSpeed)?;

        Ok(weapon_attack_speed.computed_duration(self.attack_speed))
    }

    pub fn weapon_projectile_speed(&self, weapon: &Weapon) -> Result<f32, &str> {
        let weapon_projectile_speed = self
            .weapon_build(weapon)?
            .get_stat(&Statistic::ProjectileSpeed)?;

        Ok(weapon_projectile_speed.computed_value(self.projectile_speed))
    }

    pub fn weapon_projectile_size(&self, weapon: &Weapon) -> Result<f32, &str> {
        let weapon_projectile_size = self
            .weapon_build(weapon)?
            .get_stat(&Statistic::ProjectileSize)?;

        Ok(weapon_projectile_size.computed_value(self.projectile_size))
    }

    pub fn weapon_projectile_count(&self, weapon: &Weapon) -> Result<usize, &str> {
        let weapon_projectile_count = self
            .weapon_build(weapon)?
            .get_stat(&Statistic::ProjectileCount)?
            .base_value as usize;

        Ok(self.projectile_count + weapon_projectile_count)
    }

    pub fn weapon_bounces(&self, weapon: &Weapon) -> Result<usize, &str> {
        let bounces = self
            .weapon_build(weapon)?
            .get_stat(&Statistic::Bounces)?
            .base_value as usize;

        Ok(bounces)
    }

    pub fn weapon_piercing(&self, weapon: &Weapon) -> Result<usize, &str> {
        let bounces = self
            .weapon_build(weapon)?
            .get_stat(&Statistic::Piercing)?
            .base_value as usize;

        Ok(bounces)
    }

    pub fn empty_weapon_slots(&self) -> usize {
        MAX_WEAPONS - self.weapon_builds.len()
    }

    pub fn empty_manuscript_slots(&self) -> usize {
        MAX_MANUSCRIPTS - self.manuscripts.len()
    }

    pub fn available_weapons(&self) -> HashSet<Weapon> {
        Weapon::all()
            .into_iter()
            .filter(|m| !self.weapon_builds.contains_key(m))
            .collect()
    }

    pub fn available_manuscripts(&self) -> HashSet<Manuscript> {
        Manuscript::all()
            .into_iter()
            .filter(|m| !self.manuscripts.contains_key(m))
            .collect()
    }

    pub fn manuscripts(&self) -> HashSet<Manuscript> {
        self.manuscripts.keys().cloned().collect()
    }

    pub fn weapon_stat_text(&self, weapon: &Weapon, statistic: &Statistic) -> Result<String, &str> {
        let value = match statistic {
            Statistic::Range => format!("{:.2}", self.weapon_range(weapon)?),
            Statistic::Damage => format!("{:.1}", self.weapon_damage(weapon)?),
            Statistic::ProjectileSize => format!("{:.1}", self.weapon_projectile_size(weapon)?),
            Statistic::AttackSpeed => {
                format!("{:.1} s", self.weapon_cooldown(weapon)?.as_secs_f32())
            }
            Statistic::ProjectileSpeed => {
                format!("{:.1}", self.weapon_projectile_speed(weapon)?)
            }
            Statistic::ProjectileCount => self.weapon_projectile_count(weapon)?.to_string(),
            Statistic::Bounces => self.weapon_bounces(weapon)?.to_string(),
            Statistic::Piercing => self.weapon_piercing(weapon)?.to_string(),
            Statistic::MoveSpeed => "".to_string(),
        };

        Ok(value)
    }

    pub fn weapon_level(&self, weapon: &Weapon) -> usize {
        self.weapon_builds[weapon].level
    }

    pub fn manuscript_level(&self, manuscript: &Manuscript) -> usize {
        self.manuscripts[manuscript]
    }

    pub fn manuscript_stat_text(&self, manuscript: &Manuscript) -> String {
        match manuscript.statistic() {
            Statistic::AttackSpeed => format_percentage(self.attack_speed),
            Statistic::Range => format_percentage(self.range),
            Statistic::Damage => format_percentage(self.damage),
            Statistic::ProjectileSize => format_percentage(self.projectile_size),
            Statistic::ProjectileSpeed => format_percentage(self.projectile_speed),
            Statistic::MoveSpeed => format_percentage(self.move_speed),
            Statistic::ProjectileCount => (1 + self.projectile_count).to_string(),
            Statistic::Bounces => panic!("unhandled manuscript stat: Bounces"),
            Statistic::Piercing => panic!("unhandled manuscript stat: Piercing"),
        }
    }

    fn apply_manuscript_to_build(&mut self, manuscript: &Manuscript) {
        match manuscript.statistic() {
            Statistic::ProjectileCount => self.projectile_count += 1,
            Statistic::AttackSpeed => self.attack_speed += 0.09,
            Statistic::Range => self.range += 0.07,
            Statistic::Damage => self.damage += 0.1,
            Statistic::ProjectileSize => self.projectile_size += 0.1,
            Statistic::ProjectileSpeed => self.projectile_speed += 0.1,
            Statistic::MoveSpeed => self.move_speed += 0.05,
            Statistic::Bounces => panic!("unhandled manuscript stat: Bounces"),
            Statistic::Piercing => panic!("unhandled manuscript stat: Piercing"),
        };
    }
}

impl Plugin for BuildPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerBuild>();
    }
}
