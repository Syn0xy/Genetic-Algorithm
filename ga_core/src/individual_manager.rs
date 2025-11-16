use crate::{Individual, IndividualId};

pub trait IndividualManager<I: Individual> {
    fn build(&mut self, id: IndividualId) -> I;
    fn mutate(&self, individual: &mut I);
    fn crossover(&self, a: &mut I, b: &mut I);
    fn calculate_fitness(&self, individual: &mut I) -> f32;
    fn select_parent(&self, population: &Vec<I>) -> Option<I>;
}
