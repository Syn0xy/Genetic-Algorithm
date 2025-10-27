use std::time::Instant;

use ga_core::prelude::{FromIndividual, Individual, IndividualId};
use ga_ecs::prelude::{EntityCommands, EntityId, IntoEntity};
use ga_game::prelude::{Behaviour, Cercle, Life, Position, Speed, Survivor};
use ga_vec2::FVec2;

#[derive(Debug, Default, Clone)]
pub struct SurvivorIndividual {
    pub(crate) id: IndividualId,
    pub(crate) fitness: f32,
    pub(crate) genome: Vec<f32>,
}

impl SurvivorIndividual {
    pub fn new(id: IndividualId, genome: Vec<f32>) -> Self {
        Self {
            id,
            fitness: f32::NEG_INFINITY,
            genome,
        }
    }
}

impl Individual for SurvivorIndividual {
    fn id(&self) -> &IndividualId {
        &self.id
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
}

impl IntoEntity for SurvivorIndividual {
    fn build(&self, mut commands: EntityCommands<'_>) -> EntityId {
        commands
            .insert(Position(FVec2::ZERO))
            .insert(Cercle::from_individual(self))
            .insert(Speed::from_individual(self))
            .insert(Life::from_individual(self))
            .insert(Behaviour::from_individual(self))
            .insert(Survivor {
                time_alive: Instant::now(),
            })
            .id()
    }
}
