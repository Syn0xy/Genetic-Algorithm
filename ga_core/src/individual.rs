use crate::individual_manager::IndividualId;

pub trait Individual {
    fn id(&self) -> &IndividualId;
    fn fitness(&self) -> f32;
}

pub trait FromIndividual<I: Individual> {
    fn from_individual(individual: &I) -> Self;
}
