use crate::build::PlayerBuild;
use crate::transmutation::TransmutationEffect::{Add, Multiply};
use crate::transmutation::TransmutationOffer::{NewWeapon, WeaponTransmute};
use crate::ui::transmutation::{DisplayTransmutationOffers, TransmutationTextOffer};
use crate::weapon::WeaponStatistic::*;
use crate::weapon::{Weapon, WeaponStatistic};
use bevy::prelude::*;
use rand::prelude::IndexedRandom;

pub struct TransmutationPlugin;

#[derive(Event)]
pub struct DrawTransmutationOffers;

#[derive(Event)]
pub struct ApplyTransmutationOffer(pub usize);

#[derive(Clone, Debug)]
pub enum TransmutationOffer {
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

impl WeaponTransmutation {
    pub fn text(&self) -> String {
        format!("{:?} {}", self.statistic, self.effect.to_string())
    }
}

#[derive(Debug, Clone)]
pub enum TransmutationEffect {
    Add(usize),
    Multiply(f32),
}

impl TransmutationEffect {
    pub fn to_string(&self) -> String {
        match self {
            Add(count) => format!("+ {}", count),
            Multiply(multiplier) => format!("+ {}%", (100. * multiplier) as usize),
        }
    }
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

fn weapon_statistics(weapon: &Weapon) -> Vec<WeaponStatistic> {
    match weapon {
        Weapon::Knife => vec![Range, Damage, AttackSpeed, ProjectileSpeed, ProjectileCount],
    }
}

// each weapon will have different stats that can be upgraded
// each weapon should have an associated list of transmutable statistics
// TODO cut into multiple functions
fn random_weapon_transmutation_offer(weapons: Vec<Weapon>) -> Option<WeaponTransmutationOffer> {
    let Some(random_weapon) = weapons.choose(&mut rand::rng()) else {
        return None;
    };

    let weapon_statistics = weapon_statistics(random_weapon);

    // TODO weighted draw for number of Transmutation t = (1, 2, 3)

    // TODO choose t random stat from this weapon using choose_iter
    let Some(random_statistic) = weapon_statistics.choose(&mut rand::rng()) else {
        return None;
    };

    // TODO weighted draw for random multiplier (0.02, 0.05, 0.07, 0.08, 0.1, 0.15)
    let random_effect: TransmutationEffect = match random_statistic {
        ProjectileCount => Add(1),
        _ => Multiply(0.05),
    };

    let random_transmutation = WeaponTransmutation {
        statistic: *random_statistic,
        effect: random_effect,
    };

    Some(WeaponTransmutationOffer {
        weapon: *random_weapon,
        transmutations: [None, Some(random_transmutation), None],
    })
}

fn transmutation_offer_to_text(offer: TransmutationOffer) -> TransmutationTextOffer {
    match offer {
        NewWeapon(weapon) => TransmutationTextOffer {
            offer_type: "New Weapon".to_string(),
            entity_name: format!("{:?}", weapon),
            effects: [
                "Unlocks a new weapon".to_string(),
                String::default(),
                String::default(),
            ],
        },
        WeaponTransmute(weapon_transmutation) => TransmutationTextOffer {
            offer_type: "Weapon Transmutation".to_string(),
            entity_name: format!("{:?}", weapon_transmutation.weapon),
            effects: weapon_transmutation.transmutations.map(|transmutation| {
                transmutation
                    // TODO format transmutation text using player weapon real stats
                    // requires to query each player weapon components...
                    .map(|transmutation| transmutation.text())
                    .unwrap_or(String::default())
            }),
        },
    }
}

fn draw_transmutation_offers(
    _event: On<DrawTransmutationOffers>,
    player_build: Res<PlayerBuild>,
    mut commands: Commands,
) {
    // TODO allow reroll
    let Some(a) = random_weapon_transmutation_offer(player_build.weapons()) else {
        warn!("Failed to draw a random weapon transmutation");
        return;
    };

    let Some(b) = random_weapon_transmutation_offer(player_build.weapons()) else {
        warn!("Failed to draw a random weapon transmutation");
        return;
    };

    let Some(c) = random_weapon_transmutation_offer(player_build.weapons()) else {
        warn!("Failed to draw a random weapon transmutation");
        return;
    };

    commands.insert_resource(TransmutationOffers {
        a: WeaponTransmute(a.clone()),
        b: WeaponTransmute(b.clone()),
        c: WeaponTransmute(c.clone()),
    });

    commands.trigger(DisplayTransmutationOffers {
        offer_a: transmutation_offer_to_text(WeaponTransmute(a)),
        offer_b: transmutation_offer_to_text(WeaponTransmute(b)),
        offer_c: transmutation_offer_to_text(WeaponTransmute(c)),
    })
}

fn apply_weapon_transmutation_offer(transmutation: WeaponTransmutationOffer) {
    // TODO for each transmutation stat trigger an event, ex: update_projectile_count_multiplier

    info!("Applying weapon transmutation: {:?}", transmutation);
}

fn apply_transmutation_offer(
    event: On<ApplyTransmutationOffer>,
    offers: Res<TransmutationOffers>,
    player_build: ResMut<PlayerBuild>,
) {
    let offer = offers.get(event.0);

    if let NewWeapon(new_weapon) = offer {
        // TODO trigger new_weapon_unlocked event
    }

    if let WeaponTransmute(weapon_transmutation) = offer {
        // player_build.
        apply_weapon_transmutation_offer(weapon_transmutation);
    }
}

impl Plugin for TransmutationPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(draw_transmutation_offers)
            .add_observer(apply_transmutation_offer);
    }
}
