use crate::build::{PlayerBuild, Statistic};
use crate::game::PauseState;
use crate::manuscript::Manuscript;
use crate::transmutation::TransmutationEffect::{Add, Multiply};
use crate::transmutation::TransmutationOffer::*;
use crate::ui::transmutation::{DisplayTransmutationOffers, TransmutationTextOffer};
use crate::weapon::{SpawnWeapon, Weapon};
use bevy::prelude::*;
use log::warn;
use rand::distr::weighted::WeightedIndex;
use rand::prelude::*;
use std::cmp::{PartialEq, min};
use std::collections::{HashMap, HashSet};

const OFFER_COUNT: usize = 3;

pub struct TransmutationPlugin;

#[derive(Event)]
pub struct DrawTransmutationOffers;

#[derive(Event)]
pub struct ApplyTransmutationOffer(pub usize);

#[derive(Copy, Clone, PartialEq)]
pub enum TransmutationOfferType {
    NewWeapon,
    NewManuscript,
    WeaponTransmutation,
    ManuscriptTransmutation,
}

#[derive(Clone, Debug)]
pub enum TransmutationOffer {
    NewWeapon(Weapon),
    NewManuscript(Manuscript),
    WeaponTransmutation(WeaponTransmutationOffer),
    ManuscriptTransmutation(Manuscript),
}

#[derive(Clone, Debug)]
pub struct WeaponTransmutationOffer {
    pub weapon: Weapon,
    pub transmutations: [Option<Transmutation>; OFFER_COUNT],
}

#[derive(Clone, Debug)]
pub struct Transmutation {
    pub statistic: Statistic,
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
struct TransmutationOfferPool {
    offer_types_pool: Vec<TransmutationOfferType>,
    new_manuscripts_pool: HashSet<Manuscript>,
    new_weapons_pool: HashSet<Weapon>,
    weapon_stats_pool: HashMap<Weapon, HashSet<Statistic>>,
    manuscripts_pool: HashSet<Manuscript>,
    weapon_transmutation_left: usize,
}

impl TransmutationOfferPool {
    pub fn fill_pool(&mut self, build: &PlayerBuild) {
        self.fill_new_weapons(build);
        self.fill_new_manuscripts(build);
        self.fill_weapon_stats(build);
        self.fill_manuscripts(build);

        self.fill_offer_types(build);

        self.weapon_transmutation_left = self
            .offer_types_pool
            .iter()
            .filter(|&&ot| ot == TransmutationOfferType::WeaponTransmutation)
            .count();
    }

    pub fn draw_offers(&mut self) -> [TransmutationOffer; OFFER_COUNT] {
        let offer_types = self.draw_offer_types();

        [
            self.draw_offer(offer_types[0]),
            self.draw_offer(offer_types[1]),
            self.draw_offer(offer_types[2]),
        ]
    }

    pub fn draw_offer(&mut self, offer_type: TransmutationOfferType) -> TransmutationOffer {
        match offer_type {
            TransmutationOfferType::NewWeapon => NewWeapon(self.draw_new_weapon()),
            TransmutationOfferType::NewManuscript => NewManuscript(self.draw_new_manuscript()),
            TransmutationOfferType::ManuscriptTransmutation => {
                ManuscriptTransmutation(self.draw_manuscript_transmutation())
            }
            TransmutationOfferType::WeaponTransmutation => {
                WeaponTransmutation(self.draw_weapon_transmutation())
            }
        }
    }

    fn draw_offer_types(&self) -> [TransmutationOfferType; OFFER_COUNT] {
        let mut rng = rand::rng();

        // sampled without replacement
        let sampled_result =
            self.offer_types_pool
                .sample_weighted(&mut rng, OFFER_COUNT, Self::offer_type_weight);

        let Ok(sampled_weights) = sampled_result else {
            warn!("could not sample weights to draw offer types");

            return [
                TransmutationOfferType::WeaponTransmutation,
                TransmutationOfferType::WeaponTransmutation,
                TransmutationOfferType::WeaponTransmutation,
            ];
        };

        let offers: Vec<TransmutationOfferType> = sampled_weights.copied().collect();

        [offers[0], offers[1], offers[2]]
    }

    fn draw_new_weapon(&mut self) -> Weapon {
        let new_weapon = self.random_new_weapon();

        self.remove_new_weapon(&new_weapon);

        new_weapon
    }

    fn draw_new_manuscript(&mut self) -> Manuscript {
        let new_manuscript = self.random_new_manuscript();

        self.remove_new_manuscript(&new_manuscript);

        new_manuscript
    }

    fn draw_manuscript_transmutation(&mut self) -> Manuscript {
        let manuscript = self.random_manuscript();

        self.remove_manuscript(&manuscript);

        manuscript
    }

    fn draw_weapon_transmutation(&mut self) -> WeaponTransmutationOffer {
        let mut transmutations: [Option<Transmutation>; OFFER_COUNT] = [None, None, None];

        self.weapon_transmutation_left -= 1;

        let weapon = self.random_weapon();
        let stat_count = self.draw_weapon_stats_count(&weapon);

        let stats = self.draw_weapon_stats(&weapon, stat_count);

        if stats.len() != stat_count {
            warn!(
                "Stats count mismatch, stat_count: {} stats.len: {}",
                stat_count,
                stats.len()
            );
        }

        for n in 0..stat_count {
            transmutations[n] = Some(Transmutation {
                statistic: stats[n],
                effect: self.draw_transmutation_effect(&stats[n]),
            })
        }

        WeaponTransmutationOffer {
            weapon,
            transmutations,
        }
    }

    // creates a pool of possible offers
    //
    // NewManuscript should be present {manuscript slots left} times with w = 4
    // NewWeapon should be present {weapon slots left} times with w = 4
    // ManuscriptTransmutation should be present 3 times with w = 1
    // WeaponTransmutation should be present 3 times with w = 1
    fn fill_offer_types(&mut self, build: &PlayerBuild) {
        // NewWeapon offers depend on available weapons
        for _n in 0..build.empty_weapon_slots() {
            self.offer_types_pool
                .push(TransmutationOfferType::NewWeapon);
        }
        // NewManuscript offers depend on available slots
        for _n in 0..build.empty_manuscript_slots() {
            self.offer_types_pool
                .push(TransmutationOfferType::NewManuscript);
        }

        // Manuscript Transmutation depends on owned manuscripts
        for _n in 0..build.manuscripts().len() {
            self.offer_types_pool
                .push(TransmutationOfferType::ManuscriptTransmutation);
        }

        // Player can always TransmuteWeapon
        for _n in 0..OFFER_COUNT {
            self.offer_types_pool
                .push(TransmutationOfferType::WeaponTransmutation);
        }
    }

    fn fill_new_weapons(&mut self, player_build: &PlayerBuild) {
        self.new_weapons_pool = player_build.available_weapons();
    }

    fn fill_new_manuscripts(&mut self, player_build: &PlayerBuild) {
        self.new_manuscripts_pool = player_build.available_manuscripts();
    }

    fn fill_weapon_stats(&mut self, player_build: &PlayerBuild) {
        for (weapon, build) in player_build.weapon_builds() {
            self.weapon_stats_pool
                .insert(*weapon, build.statistics().into_iter().collect());
        }
    }

    fn fill_manuscripts(&mut self, player_build: &PlayerBuild) {
        self.manuscripts_pool = player_build.manuscripts().into_iter().collect();
    }

    fn random_new_weapon(&self) -> Weapon {
        self.new_weapons_pool
            .iter()
            .cloned()
            .choose(&mut rand::rng())
            .expect("No new weapons left in the pool")
    }

    fn random_new_manuscript(&self) -> Manuscript {
        self.new_manuscripts_pool
            .iter()
            .cloned()
            .choose(&mut rand::rng())
            .expect("No new manuscripts left in the pool")
    }

    fn random_weapon(&self) -> Weapon {
        self.weapon_stats_pool
            .keys()
            .cloned()
            .choose(&mut rand::rng())
            .expect("No weapons left in the pool")
    }

    fn random_manuscript(&self) -> Manuscript {
        self.manuscripts_pool
            .iter()
            .cloned()
            .choose(&mut rand::rng())
            .expect("No Manuscripts left in the pool")
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

    fn draw_weapon_stats_count(&self, weapon: &Weapon) -> usize {
        let weapon_sat = self.weapon_stats(weapon);
        let reserved_budget = self.total_stats() - self.weapon_transmutation_left;
        let budget = min(weapon_sat, reserved_budget);

        let stat_options: Vec<usize> = [1, 2, 3].into_iter().take(budget).collect();
        let stat_weights: Vec<usize> = [80, 15, 5].into_iter().take(budget).collect();

        let mut rng = rand::rng();
        let weighted_index = WeightedIndex::new(&stat_weights).unwrap();
        let stat_count_index = weighted_index.sample(&mut rng);

        stat_options[stat_count_index]
    }

    fn remove_new_weapon(&mut self, weapon: &Weapon) {
        self.new_weapons_pool.remove(weapon);
    }

    fn remove_new_manuscript(&mut self, manuscript: &Manuscript) {
        self.new_manuscripts_pool.remove(manuscript);
    }

    fn remove_weapon_stats(&mut self, weapon: &Weapon, stats: &Vec<Statistic>) {
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

    fn remove_manuscript(&mut self, manuscript: &Manuscript) {
        self.manuscripts_pool.remove(manuscript);
    }

    fn draw_weapon_stats(&mut self, weapon: &Weapon, stat_count: usize) -> Vec<Statistic> {
        let available_stats: Vec<Statistic> =
            self.weapon_stats_pool[weapon].clone().into_iter().collect();

        let mut rng = rand::rng();

        // sampled without replacement
        let sampled_result =
            available_stats.sample_weighted(&mut rng, stat_count, Self::statistic_weight);

        let Ok(sampled_weights) = sampled_result else {
            warn!("could not sample weights to draw weapon stats");
            return Vec::new();
        };

        let stats: Vec<Statistic> = sampled_weights.copied().collect();

        self.remove_weapon_stats(weapon, &stats);

        stats
    }

    fn draw_transmutation_effect(&self, stat: &Statistic) -> TransmutationEffect {
        if stat.is_incremental() {
            let amounts = [1, 2, 3];
            let weights = [80, 8, 2];

            let mut rng = rand::rng();
            let weighted_index = WeightedIndex::new(weights).unwrap();
            let index = weighted_index.sample(&mut rng);

            return Add(amounts[index]);
        }

        let multipliers = [0.02, 0.04, 0.06, 0.08, 0.10, 0.12];
        let weights = [1, 5, 10, 7, 2, 1];

        let mut rng = rand::rng();
        let weighted_index = WeightedIndex::new(weights).unwrap();
        let index = weighted_index.sample(&mut rng);

        Multiply(multipliers[index])
    }

    fn offer_type_weight(offer_type: &TransmutationOfferType) -> f64 {
        match offer_type {
            TransmutationOfferType::NewManuscript => 2.,
            TransmutationOfferType::NewWeapon => 2.,
            TransmutationOfferType::WeaponTransmutation => 1.,
            TransmutationOfferType::ManuscriptTransmutation => 1.,
        }
    }

    fn statistic_weight(stat: &Statistic) -> f64 {
        match stat {
            Statistic::Bounces => 5.,
            Statistic::Damage => 5.,
            Statistic::AttackSpeed => 2.,
            Statistic::ProjectileCount => 1.,
            _ => 3.,
        }
    }
}

fn weapon_transmutation_to_text(
    weapon: &Weapon,
    transmutation: &Transmutation,
    player_build: &PlayerBuild,
    transmuted_build: &PlayerBuild,
) -> Result<String, String> {
    let current_stat_text = player_build.weapon_stat_text(weapon, &transmutation.statistic)?;
    let transmuted_stat_text =
        transmuted_build.weapon_stat_text(weapon, &transmutation.statistic)?;

    Ok(format!(
        "{:?} {} -> {}",
        transmutation.statistic, current_stat_text, transmuted_stat_text
    ))
}

fn new_weapon_to_text(weapon: &Weapon) -> TransmutationTextOffer {
    TransmutationTextOffer {
        offer_type: "New Weapon".to_string(),
        item_sprite_index: weapon.sprite_index(),
        entity_name: format!("{:?}", weapon),
        effects: [
            format!("Unlock {:?} Weapon", weapon),
            String::default(),
            String::default(),
        ],
    }
}

fn new_manuscript_to_text(manuscript: &Manuscript) -> TransmutationTextOffer {
    TransmutationTextOffer {
        offer_type: "New Manuscript".to_string(),
        item_sprite_index: manuscript.sprite_index(),
        entity_name: format!("{:?}", manuscript),
        effects: [
            format!("Unlock {:?} Manuscript", manuscript),
            String::default(),
            String::default(),
        ],
    }
}

fn weapon_transmutation_offer_to_text(
    offer: &WeaponTransmutationOffer,
    player_build: &PlayerBuild,
) -> TransmutationTextOffer {
    let mut transmuted_build: PlayerBuild = player_build.clone();
    let mut effects = [String::default(), String::default(), String::default()];

    transmuted_build.apply_weapon_transmutation(offer);

    for (index, transmutation) in offer.transmutations.iter().flatten().enumerate() {
        effects[index] = weapon_transmutation_to_text(
            &offer.weapon,
            transmutation,
            player_build,
            &transmuted_build,
        )
        .unwrap_or_default();
    }

    let weapon_level = transmuted_build.weapon_level(&offer.weapon);

    TransmutationTextOffer {
        offer_type: "Weapon Transmutation".to_string(),
        item_sprite_index: offer.weapon.sprite_index(),
        entity_name: format!("{:?} +{}", offer.weapon, weapon_level),
        effects,
    }
}

fn manuscript_transmutation_to_text(
    manuscript: &Manuscript,
    player_build: &PlayerBuild,
) -> TransmutationTextOffer {
    let mut transmuted_build: PlayerBuild = player_build.clone();
    let mut effects = [String::default(), String::default(), String::default()];

    transmuted_build.apply_manuscript_transmutation(manuscript);
    let transmutation_level = transmuted_build.manuscript_level(manuscript);

    let current_stat_text = player_build.manuscript_stat_text(manuscript);
    let transmuted_stat_text = transmuted_build.manuscript_stat_text(manuscript);

    effects[0] = format!(
        "{:?} {} -> {}",
        manuscript.statistic(),
        current_stat_text,
        transmuted_stat_text
    );

    TransmutationTextOffer {
        offer_type: "Manuscript Transmutation".to_string(),
        item_sprite_index: manuscript.sprite_index(),
        entity_name: format!("{:?} +{}", manuscript, transmutation_level),
        effects,
    }
}

fn transmutation_offer_to_text(
    player_build: &PlayerBuild,
    offer: &TransmutationOffer,
) -> TransmutationTextOffer {
    match offer {
        NewWeapon(weapon) => new_weapon_to_text(weapon),
        NewManuscript(manuscript) => new_manuscript_to_text(manuscript),
        WeaponTransmutation(weapon_transmutation) => {
            weapon_transmutation_offer_to_text(weapon_transmutation, player_build)
        }
        ManuscriptTransmutation(manuscript) => {
            manuscript_transmutation_to_text(manuscript, player_build)
        }
    }
}

fn draw_transmutation_offers(
    _event: On<DrawTransmutationOffers>,
    player_build: Res<PlayerBuild>,
    mut next_state: ResMut<NextState<PauseState>>,
    mut commands: Commands,
) {
    next_state.set(PauseState(true));

    let mut offer_pool = TransmutationOfferPool::default();

    offer_pool.fill_pool(&player_build);

    let offers = offer_pool.draw_offers();

    let offer_text_a = transmutation_offer_to_text(&player_build, &offers[0]);
    let offer_text_b = transmutation_offer_to_text(&player_build, &offers[1]);
    let offer_text_c = transmutation_offer_to_text(&player_build, &offers[2]);

    commands.insert_resource(TransmutationOffers {
        a: offers[0].clone(),
        b: offers[1].clone(),
        c: offers[2].clone(),
    });

    commands.trigger(DisplayTransmutationOffers {
        offer_a: offer_text_a,
        offer_b: offer_text_b,
        offer_c: offer_text_c,
    });
}

fn apply_transmutation_offer(
    event: On<ApplyTransmutationOffer>,
    offers: Res<TransmutationOffers>,
    mut player_build: ResMut<PlayerBuild>,
    mut commands: Commands,
) {
    let offer = offers.get(event.0);

    match offer {
        NewWeapon(weapon) => {
            info!("Adding weapon {:?}", weapon);
            commands.trigger(SpawnWeapon(weapon));
        }
        NewManuscript(manuscript) => {
            info!("Adding manuscript {:?}", manuscript);
            player_build.add_manuscript(&manuscript);
        }
        WeaponTransmutation(transmutation) => {
            info!("Applying weapon transmutation: {:?}", transmutation);

            player_build.apply_weapon_transmutation(&transmutation)
        }
        ManuscriptTransmutation(manuscript) => {
            info!("Applying manuscript transmutation {:?}", manuscript);
            player_build.apply_manuscript_transmutation(&manuscript);
        }
    }

    info!("player build state: {:?}", player_build);
}

impl Plugin for TransmutationPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(draw_transmutation_offers)
            .add_observer(apply_transmutation_offer);
    }
}
