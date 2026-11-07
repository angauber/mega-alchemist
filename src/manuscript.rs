use crate::transmutation::{Transmutation, TransmutationEffect};
use crate::weapon::Statistic;
use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Manuscript {
    Strength,
    Tempo,
    Chaos,
    Agility,
}

impl Manuscript {
    pub fn all() -> Vec<Manuscript> {
        vec![
            Manuscript::Strength,
            Manuscript::Tempo,
            Manuscript::Chaos,
            Manuscript::Agility,
        ]
    }

    pub fn statistic(&self) -> Statistic {
        match self {
            Manuscript::Strength => Statistic::Damage,
            Manuscript::Tempo => Statistic::AttackSpeed,
            Manuscript::Chaos => Statistic::ProjectileCount,
            Manuscript::Agility => Statistic::MoveSpeed,
        }
    }

    #[allow(dead_code)]
    pub fn effect(&self) -> TransmutationEffect {
        match self {
            Manuscript::Chaos => TransmutationEffect::Add(1),
            Manuscript::Agility => TransmutationEffect::Multiply(0.05),
            _ => TransmutationEffect::Multiply(0.1),
        }
    }

    #[allow(dead_code)]
    pub fn transmutation(&self) -> Transmutation {
        Transmutation {
            statistic: self.statistic(),
            effect: self.effect(),
        }
    }
}
