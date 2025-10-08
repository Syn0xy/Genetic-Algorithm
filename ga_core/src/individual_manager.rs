use crate::prelude::Individual;

pub type IndividualId = u32;

pub trait IndividualManager {
    type IndType: Individual;

    fn build(&mut self) -> Self::IndType;
    fn mutate(&self, individual: &mut Self::IndType);
    fn crossover(&self, a: &mut Self::IndType, b: &mut Self::IndType);
    fn calculate_fitness(&self, individual: &mut Self::IndType) -> f32;
    fn select_parent(&self, population: &Vec<Self::IndType>) -> Option<Self::IndType>;
}
