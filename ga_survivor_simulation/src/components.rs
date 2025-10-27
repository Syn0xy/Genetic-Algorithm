use ga_core::prelude::FromIndividual;
use ga_game::prelude::{Behaviour, Cercle, Life, Speed};

use crate::prelude::SurvivorIndividual;

impl FromIndividual<SurvivorIndividual> for Cercle {
    fn from_individual(individual: &SurvivorIndividual) -> Self {
        Self(individual.genome[3])
    }
}

impl FromIndividual<SurvivorIndividual> for Speed {
    fn from_individual(individual: &SurvivorIndividual) -> Self {
        Self(individual.genome[2])
    }
}

impl FromIndividual<SurvivorIndividual> for Life {
    fn from_individual(individual: &SurvivorIndividual) -> Self {
        Self(individual.genome[1].round() as u8)
    }
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
