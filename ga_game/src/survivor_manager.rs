use std::mem;

use ga_core::prelude::{GeneSpec, IndividualId, IndividualManager};
use rand::{Rng, seq::IndexedRandom};

use crate::prelude::SurvivorIndividual;

pub struct SurvivorManager {
    individual_id_sequence: IndividualId,
    tournament_size: usize,
    genes: Vec<GeneSpec>,
}

impl SurvivorManager {
    pub fn new(tournament_size: usize, genes: Vec<GeneSpec>) -> Self {
        Self {
            individual_id_sequence: Default::default(),
            tournament_size,
            genes,
        }
    }

    fn create_individual(&mut self) -> IndividualId {
        let individual_id = self.individual_id_sequence;
        self.individual_id_sequence += 1;
        individual_id
    }
}

impl IndividualManager for SurvivorManager {
    type IndType = SurvivorIndividual;

    fn build(&mut self) -> Self::IndType {
        Self::IndType::new(
            self.create_individual(),
            self.genes
                .iter()
                .map(|spec| spec.init_range.random_value())
                .collect(),
        )
    }

    fn mutate(&self, individual: &mut Self::IndType) {
        for (i, spec) in self.genes.iter().enumerate() {
            if rand::rng().random_bool(spec.mutation_rate as f64) {
                let delta = spec.mut_range.random_value();
                let new_gene = spec.clamp(individual.genome[i] + delta);

                individual.genome[i] = new_gene;
            }
        }
    }

    fn crossover(&self, a: &mut Self::IndType, b: &mut Self::IndType) {
        for i in 0..self.genes.len() {
            if rand::random() {
                mem::swap(&mut a.genome[i], &mut b.genome[i]);
            }
        }
    }

    fn calculate_fitness(&self, individual: &mut Self::IndType) -> f32 {
        individual.fitness = individual.genome.iter().sum();
        individual.fitness
    }

    fn select_parent(&self, population: &Vec<Self::IndType>) -> Option<Self::IndType> {
        (0..self.tournament_size)
            .filter_map(|_| random_element(population))
            .max_by(|a, b| {
                a.fitness
                    .partial_cmp(&b.fitness)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .cloned()
    }
}

fn random_element<T>(population: &Vec<T>) -> Option<&T> {
    population.choose(&mut rand::rng())
}
