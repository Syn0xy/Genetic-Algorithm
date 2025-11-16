use std::fmt::Debug;

use crate::{IndividualId, individual::Individual, individual_manager::IndividualManager};

pub struct Simulation<I: Individual, IM: IndividualManager<I>> {
    manager: IM,
    generation: u32,
    individual_id_sequence: IndividualId,
    population: Vec<I>,
}

impl<I, IM> Default for Simulation<I, IM>
where
    I: Individual,
    IM: IndividualManager<I> + Default,
{
    fn default() -> Self {
        Self {
            manager: Default::default(),
            generation: Default::default(),
            individual_id_sequence: Default::default(),
            population: Default::default(),
        }
    }
}

impl<I, IM> Simulation<I, IM>
where
    I: Individual,
    IM: IndividualManager<I>,
{
    pub fn new(manager: IM) -> Self {
        Self {
            generation: 0,
            manager,
            individual_id_sequence: Default::default(),
            population: Default::default(),
        }
    }

    pub fn population(&self) -> &Vec<I> {
        &self.population
    }

    pub fn create_individual(&mut self) -> IndividualId {
        let individual_id = self.individual_id_sequence;
        self.individual_id_sequence += 1;
        individual_id
    }

    pub fn initialize_population(&mut self, population_size: usize) {
        self.individual_id_sequence = IndividualId::default();
        self.population.clear();

        for _ in 0..population_size {
            let individual_id = self.create_individual();
            let individual = self.manager.build(individual_id);

            self.population.push(individual);
        }
    }

    pub fn generation(&mut self) {
        let mut new_population = Vec::new();

        for _ in 0..self.population.len() / 2 {
            let (Some(mut p1), Some(mut p2)) = (
                self.manager.select_parent(&self.population),
                self.manager.select_parent(&self.population),
            ) else {
                continue;
            };

            self.manager.crossover(&mut p1, &mut p2);
            self.manager.mutate(&mut p1);
            self.manager.mutate(&mut p2);
            self.manager.calculate_fitness(&mut p1);
            self.manager.calculate_fitness(&mut p2);

            new_population.push(p1);
            new_population.push(p2);
        }

        self.generation += 1;
        self.population = new_population;
    }
}

impl<I, IM> Simulation<I, IM>
where
    I: Individual + Debug,
    IM: IndividualManager<I>,
{
    pub fn print_result(&self) {
        let mut buffer = String::new();

        buffer.push_str(&format!("Génération : {}\n", self.generation));

        for individual in self.population.iter() {
            buffer.push_str(&format!(" - {:?}\n", individual));
        }

        println!("{}", buffer);
    }
}
