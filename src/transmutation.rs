use crate::weapon::WeaponType;
use bevy::prelude::*;

pub struct Transmutation {
    pub statistic: TransmutationStatistic,
    pub effect: TransmutationEffect,
}

impl Transmutation {
    pub fn text(&self) -> String {
        format!("{:?} {:?}", self.statistic, self.effect)
    }
}

#[derive(Debug)]
pub enum TransmutationStatistic {
    Range,
    Damage,
    AttackSpeed,
    ProjectileSpeed,
    ProjectileCount,
}

#[derive(Debug)]
pub enum TransmutationEffect {
    Add(usize),
    Multiply(f32),
}

// NewWeaponOffer
pub struct WeaponTransmutationOffer {
    pub identifier: usize,
    pub weapon: WeaponType,
    pub transmutations: [Option<Transmutation>; 3],
}

// each weapon will have different stats that can be upgraded
// each weapon should have an associated list of transmutable statistics
pub fn random_weapon_transmutation() {
    // get all player weapons
    // pick a random ome
    // pick a random stat
}
