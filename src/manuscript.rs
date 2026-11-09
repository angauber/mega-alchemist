use crate::build::Statistic;
use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Manuscript {
    Strength,
    Dexterity,
    Quantity,
    Agility,
    Vision,
    Vitality,
}

impl Manuscript {
    pub fn all() -> Vec<Manuscript> {
        vec![
            Manuscript::Strength,
            Manuscript::Dexterity,
            Manuscript::Quantity,
            Manuscript::Agility,
            Manuscript::Vision,
            Manuscript::Vitality,
        ]
    }

    pub fn statistic(&self) -> Statistic {
        match self {
            Manuscript::Strength => Statistic::Damage,
            Manuscript::Dexterity => Statistic::AttackSpeed,
            Manuscript::Quantity => Statistic::ProjectileCount,
            Manuscript::Agility => Statistic::MoveSpeed,
            Manuscript::Vision => Statistic::Range,
            Manuscript::Vitality => Statistic::Regen,
        }
    }

    pub fn sprite_index(&self) -> usize {
        match self {
            Manuscript::Strength => 231,
            Manuscript::Dexterity => 232,
            Manuscript::Quantity => 234,
            Manuscript::Agility => 235,
            Manuscript::Vision => 233,
            Manuscript::Vitality => 236,
        }
    }
}
