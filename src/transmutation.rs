use crate::build::PlayerBuild;
use crate::transmutation::TransmutationEffect::{Add, Multiply};
use crate::transmutation::TransmutationOffer::{NewWeapon, WeaponTransmute};
use crate::ui::transmutation::{DisplayTransmutationOffers, TransmutationTextOffer};
use crate::weapon::{Weapon, WeaponStatistic};
use bevy::prelude::*;
use log::warn;
use rand::distr::weighted::WeightedIndex;
use rand::prelude::*;
use std::cmp::min;
use std::collections::{HashMap, HashSet};

pub struct TransmutationPlugin;

#[derive(Event)]
pub struct DrawTransmutationOffers;

#[derive(Event)]
pub struct ApplyTransmutationOffer(pub usize);

#[derive(Clone, Debug)]
pub enum TransmutationOffer {
    #[allow(dead_code)]
    NewWeapon(Weapon),
    WeaponTransmute(WeaponTransmutationOffer),
}

#[derive(Clone, Debug)]
pub struct WeaponTransmutationOffer {
    pub weapon: Weapon,
    pub transmutations: [Option<WeaponTransmutation>; 3],
}

#[derive(Clone, Debug)]
pub struct WeaponTransmutation {
    pub statistic: WeaponStatistic,
    pub effect: TransmutationEffect,
}

#[derive(Debug, Clone)]
pub enum TransmutationEffect {
    Add(usize),
    Multiply(f32),
}

#[derive(Resource, Clone, Debug)]
pub struct TransmutationOffers {
    a: TransmutationOffer,
    b: TransmutationOffer,
    c: TransmutationOffer,
}

impl TransmutationOffers {
    pub fn get(&self, identifier: usize) -> TransmutationOffer {
        match identifier {
            0 => self.a.clone(),
            1 => self.b.clone(),
            _ => self.c.clone(),
        }
    }
}

#[derive(Default)]
struct WeaponTransmutationOfferPool {
    weapon_stats_pool: HashMap<Weapon, HashSet<WeaponStatistic>>,
}

impl WeaponTransmutationOfferPool {
    pub fn fill_pool(&mut self, player_build: &PlayerBuild) {
        for weapon_build in player_build.weapon_builds() {
            self.weapon_stats_pool.insert(
                weapon_build.weapon,
                weapon_build.statistics().into_iter().collect(),
            );
        }
    }

    fn random_weapon(&self) -> Result<Weapon, &str> {
        self.weapon_stats_pool
            .keys()
            .cloned()
            .choose(&mut rand::rng())
            .ok_or("No weapons left in the pool")
    }

    fn weapon_stats(&self, weapon: &Weapon) -> usize {
        self.weapon_stats_pool[weapon].len()
    }

    fn total_stats(&self) -> usize {
        self.weapon_stats_pool
            .values()
            .map(|stats| stats.len())
            .sum()
    }

    fn random_stat_count(&self, weapon: &Weapon, offers_left: usize) -> usize {
        let weapon_sat = self.weapon_stats(weapon);
        let reserved_budget = self.total_stats() - offers_left;
        let budget = min(weapon_sat, reserved_budget);

        let stat_options: Vec<usize> = [1, 2].into_iter().take(budget).collect();
        let stat_weights: Vec<usize> = [8, 2].into_iter().take(budget).collect();

        let mut rng = rand::rng();
        let weighted_index = WeightedIndex::new(&stat_weights).unwrap();
        let stat_count_index = weighted_index.sample(&mut rng);

        stat_options[stat_count_index]
    }

    fn remove_weapon_stats(&mut self, weapon: &Weapon, stats: &Vec<WeaponStatistic>) {
        let Some(weapon_stats) = self.weapon_stats_pool.get_mut(weapon) else {
            warn!("Weapon not in the pool");
            return;
        };

        for stat in stats {
            weapon_stats.remove(stat);
        }

        if weapon_stats.is_empty() {
            self.weapon_stats_pool.remove(weapon);
        }
    }

    fn statistic_weight(stat: &WeaponStatistic) -> f64 {
        match stat {
            WeaponStatistic::Damage => 5.,
            WeaponStatistic::AttackSpeed => 2.,
            WeaponStatistic::ProjectileCount => 1.,
            _ => 3.,
        }
    }

    fn draw_random_weapon_stats(
        &mut self,
        weapon: &Weapon,
        stat_count: usize,
    ) -> Vec<WeaponStatistic> {
        let available_stats: Vec<WeaponStatistic> =
            self.weapon_stats_pool[weapon].clone().into_iter().collect();

        let mut rng = rand::rng();

        let sampled_result =
            available_stats.sample_weighted(&mut rng, stat_count, Self::statistic_weight);

        let Ok(sampled_weights) = sampled_result else {
            warn!("could not sample weights to draw weapon stats");
            return Vec::new();
        };

        let stats: Vec<WeaponStatistic> = sampled_weights.copied().collect();

        self.remove_weapon_stats(weapon, &stats);

        stats
    }

    pub fn draw_transmutation_effect(&self, stat: &WeaponStatistic) -> TransmutationEffect {
        if stat == &WeaponStatistic::ProjectileCount {
            let amounts = [1, 2, 3];
            let weights = [20, 5, 1];

            let mut rng = rand::rng();
            let weighted_index = WeightedIndex::new(weights).unwrap();
            let index = weighted_index.sample(&mut rng);

            return Add(amounts[index]);
        }

        let multipliers = [0.02, 0.04, 0.06, 0.08, 0.10];
        let weights = [1, 5, 10, 5, 1];

        let mut rng = rand::rng();
        let weighted_index = WeightedIndex::new(weights).unwrap();
        let index = weighted_index.sample(&mut rng);

        Multiply(multipliers[index])
    }

    pub fn draw_unique_offer(
        &mut self,
        offers_left: usize,
    ) -> Result<WeaponTransmutationOffer, String> {
        let mut transmutations: [Option<WeaponTransmutation>; 3] = [None, None, None];

        let weapon = self.random_weapon()?;
        let stat_count = self.random_stat_count(&weapon, offers_left);

        let stats = self.draw_random_weapon_stats(&weapon, stat_count);

        if stats.len() != stat_count {
            return Err(format!(
                "Stats count mismatch, stat_count: {} stats.len: {}",
                stat_count,
                stats.len()
            ));
        }

        for n in 0..stat_count {
            transmutations[n] = Some(WeaponTransmutation {
                statistic: stats[n],
                effect: self.draw_transmutation_effect(&stats[n]),
            })
        }

        Ok(WeaponTransmutationOffer {
            weapon,
            transmutations,
        })
    }
}

fn new_weapon_to_text(weapon: &Weapon) -> TransmutationTextOffer {
    TransmutationTextOffer {
        offer_type: "New Weapon".to_string(),
        entity_name: format!("{:?}", weapon),
        effects: [
            "Unlocks a new weapon".to_string(),
            String::default(),
            String::default(),
        ],
    }
}
fn weapon_transmutation_to_text(
    weapon: &Weapon,
    transmutation: &WeaponTransmutation,
    player_build: &PlayerBuild,
    transmuted_build: &PlayerBuild,
) -> Result<String, String> {
    let current_stat_text = player_build.stat_text(weapon, &transmutation.statistic)?;
    let transmuted_stat_text = transmuted_build.stat_text(weapon, &transmutation.statistic)?;

    Ok(format!(
        "{:?} {} -> {}",
        transmutation.statistic, current_stat_text, transmuted_stat_text
    ))
}

fn weapon_transmutation_offer_to_text(
    weapon_transmutation: WeaponTransmutationOffer,
    player_build: &PlayerBuild,
) -> TransmutationTextOffer {
    let mut transmuted_build: PlayerBuild = player_build.clone();

    transmuted_build.apply_weapon_transmutation(&weapon_transmutation);

    let effects = weapon_transmutation.transmutations.map(|transmutation| {
        transmutation
            .map(|transmutation| {
                weapon_transmutation_to_text(
                    &weapon_transmutation.weapon,
                    &transmutation,
                    player_build,
                    &transmuted_build,
                )
                .unwrap_or_default()
            })
            .unwrap_or_default()
    });

    TransmutationTextOffer {
        offer_type: "Weapon Transmutation".to_string(),
        entity_name: format!("{:?}", weapon_transmutation.weapon),
        effects,
    }
}

fn transmutation_offer_to_text(
    player_build: &PlayerBuild,
    offer: TransmutationOffer,
) -> TransmutationTextOffer {
    match offer {
        NewWeapon(weapon) => new_weapon_to_text(&weapon),
        WeaponTransmute(weapon_transmutation) => {
            weapon_transmutation_offer_to_text(weapon_transmutation, player_build)
        }
    }
}

fn draw_transmutation_offers(
    _event: On<DrawTransmutationOffers>,
    player_build: Res<PlayerBuild>,
    mut commands: Commands,
) {
    let mut weapon_offer_pool = WeaponTransmutationOfferPool::default();

    weapon_offer_pool.fill_pool(&player_build);

    let Ok(a) = weapon_offer_pool.draw_unique_offer(2) else {
        warn!("Failed to draw a random weapon transmutation");
        return;
    };

    let Ok(b) = weapon_offer_pool.draw_unique_offer(1) else {
        warn!("Failed to draw a random weapon transmutation");
        return;
    };

    let Ok(c) = weapon_offer_pool.draw_unique_offer(0) else {
        warn!("Failed to draw a random weapon transmutation");
        return;
    };

    commands.insert_resource(TransmutationOffers {
        a: WeaponTransmute(a.clone()),
        b: WeaponTransmute(b.clone()),
        c: WeaponTransmute(c.clone()),
    });

    commands.trigger(DisplayTransmutationOffers {
        offer_a: transmutation_offer_to_text(&player_build, WeaponTransmute(a)),
        offer_b: transmutation_offer_to_text(&player_build, WeaponTransmute(b)),
        offer_c: transmutation_offer_to_text(&player_build, WeaponTransmute(c)),
    })
}

fn apply_transmutation_offer(
    event: On<ApplyTransmutationOffer>,
    offers: Res<TransmutationOffers>,
    mut player_build: ResMut<PlayerBuild>,
) {
    let offer = offers.get(event.0);

    if let NewWeapon(_new_weapon) = offer {
        // TODO trigger unlock_new_weapon event when more weapons are added
    }

    if let WeaponTransmute(transmutation) = offer {
        info!("Applying weapon transmutation: {:?}", transmutation);

        player_build.apply_weapon_transmutation(&transmutation)
    }
}

impl Plugin for TransmutationPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(draw_transmutation_offers)
            .add_observer(apply_transmutation_offer);
    }
}
