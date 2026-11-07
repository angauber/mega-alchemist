use crate::build::Statistic;
use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Manuscript {
    Strength,
    Dexterity,
    Quantity,
    Agility,
    Vision,
}

impl Manuscript {
    pub fn all() -> Vec<Manuscript> {
        vec![
            Manuscript::Strength,
            Manuscript::Dexterity,
            Manuscript::Quantity,
            Manuscript::Agility,
            Manuscript::Vision,
        ]
    }

    pub fn statistic(&self) -> Statistic {
        match self {
            Manuscript::Strength => Statistic::Damage,
            Manuscript::Dexterity => Statistic::AttackSpeed,
            Manuscript::Quantity => Statistic::ProjectileCount,
            Manuscript::Agility => Statistic::MoveSpeed,
            Manuscript::Vision => Statistic::Range,
        }
    }
}
