use std::time::Instant;

use ga_core::prelude::{Individual, IndividualId};
use ga_vec2::FVec2;

use crate::prelude::SurvivorIndividual;

pub trait FromIndividual<I: Individual> {
    fn from_individual(individual: &I) -> Self;
}

#[derive(Debug, Clone, Copy)]
pub struct Position(pub(crate) FVec2);

#[derive(Debug)]
pub struct RandomMovement {
    pub(crate) direction: FVec2,
}

#[derive(Debug)]
pub struct Life(pub(crate) u8);

#[derive(Debug, Clone, Copy)]
pub struct Cercle(pub(crate) f32);

#[derive(Debug)]
pub struct Speed(pub(crate) f32);

#[derive(Debug, Clone, Copy)]
pub struct Enemy {
    pub(crate) attack: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct Food(pub(crate) u8);

#[derive(Debug)]
pub enum Behaviour {
    Fearful,
    Hungry,
    Random,
}

#[derive(Debug)]
pub struct Survivor {
    pub(crate) individual_id: IndividualId,
    pub(crate) time_alive: Instant,
}

impl FromIndividual<SurvivorIndividual> for Behaviour {
    fn from_individual(individual: &SurvivorIndividual) -> Self {
        let behaviour_value = individual.genome[0].round() as u8;

        match behaviour_value {
            0 => Behaviour::Random,
            1 => Behaviour::Hungry,
            _ => Behaviour::Fearful,
        }
    }
}

impl FromIndividual<SurvivorIndividual> for Life {
    fn from_individual(individual: &SurvivorIndividual) -> Self {
        Self(individual.genome[1].round() as u8)
    }
}

impl FromIndividual<SurvivorIndividual> for Speed {
    fn from_individual(individual: &SurvivorIndividual) -> Self {
        Self(individual.genome[2])
    }
}

impl FromIndividual<SurvivorIndividual> for Cercle {
    fn from_individual(individual: &SurvivorIndividual) -> Self {
        Self(individual.genome[3])
    }
}

impl FromIndividual<SurvivorIndividual> for Survivor {
    fn from_individual(individual: &SurvivorIndividual) -> Self {
        Self {
            individual_id: individual.id,
            time_alive: Instant::now(),
        }
    }
}

impl Life {
    pub(crate) fn eat(&mut self, food: &Food) {
        self.0 += food.0;
    }
}
