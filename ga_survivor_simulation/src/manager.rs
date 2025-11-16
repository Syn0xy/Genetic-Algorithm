use std::{cmp::Ordering, mem};

use ga_core::{Gene, Individual, IndividualId, IndividualManager};
use rand::{Rng, seq::IndexedRandom};

use crate::{SurvivorGene, SurvivorIndividual};

const TOURNAMENT_SIZE: usize = 4;
const SURVIVOR_GENOME: [SurvivorGene; 4] = [
    SurvivorGene::Behaviour,
    SurvivorGene::Health,
    SurvivorGene::Speed,
    SurvivorGene::Size,
];

#[derive(Default)]
pub struct SurvivorManager;

impl IndividualManager<SurvivorIndividual> for SurvivorManager {
    fn build(&mut self, id: IndividualId) -> SurvivorIndividual {
        SurvivorIndividual::new(id, create_genome())
    }

    fn mutate(&self, individual: &mut SurvivorIndividual) {
        for (spec, gene) in SURVIVOR_GENOME.into_iter().zip(&mut individual.genome) {
            if rand::rng().random_bool(spec.mutation_rate()) {
                *gene = spec.clamp(*gene + spec.random_mutation());
            }
        }
    }

    fn crossover(&self, a: &mut SurvivorIndividual, b: &mut SurvivorIndividual) {
        for (gene_a, gene_b) in a.genome.iter_mut().zip(&mut b.genome) {
            if rand::random() {
                mem::swap(gene_a, gene_b);
            }
        }
    }

    fn calculate_fitness(&self, individual: &mut SurvivorIndividual) -> f32 {
        individual.fitness = individual.genome.iter().sum();
        individual.fitness
    }

    fn select_parent(&self, population: &Vec<SurvivorIndividual>) -> Option<SurvivorIndividual> {
        (0..TOURNAMENT_SIZE)
            .filter_map(|_| random_element(population))
            .max_by(individual_sort)
            .cloned()
    }
}

fn create_genome() -> Vec<f32> {
    SURVIVOR_GENOME
        .into_iter()
        .map(|spec| spec.random_global())
        .collect()
}

fn individual_sort<T: Individual>(a: &&T, b: &&T) -> Ordering {
    a.fitness().total_cmp(&b.fitness())
}

fn random_element<T>(slice: &Vec<T>) -> Option<&T> {
    slice.choose(&mut rand::rng())
}
