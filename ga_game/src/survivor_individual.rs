use ga_core::prelude::{Individual, IndividualId};

#[derive(Debug, Default, Clone)]
pub struct SurvivorIndividual {
    pub id: IndividualId,
    pub fitness: f32,
    pub genome: Vec<f32>,
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
