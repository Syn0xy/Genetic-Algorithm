use std::fmt::Debug;

use crate::{individual::Individual, individual_manager::IndividualManager, prelude::IndividualId};

pub trait Simulation<I: Individual> {
    fn population(&self) -> &Vec<I>;
    fn initialize_population(&mut self);
    fn generation(&mut self);
    fn print_result(&self);
}

pub struct SimulationHandler<I: Individual, IM: IndividualManager<I>> {
    manager: IM,
    generation: u32,
    population_size: usize,
    population: Vec<I>,
    individual_id_sequence: IndividualId,
}

impl<I, IM> SimulationHandler<I, IM>
where
    I: Individual,
    IM: IndividualManager<I>,
{
    pub fn new(manager: IM, population_size: usize) -> Self {
        Self {
            generation: 0,
            manager,
            population_size,
            population: Default::default(),
            individual_id_sequence: Default::default(),
        }
    }

    fn create_individual(&mut self) -> IndividualId {
        let individual_id = self.individual_id_sequence;
        self.individual_id_sequence += 1;
        individual_id
    }
}

impl<I, IM> Simulation<I> for SimulationHandler<I, IM>
where
    I: Individual + Debug,
    IM: IndividualManager<I>,
{
    fn population(&self) -> &Vec<I> {
        &self.population
    }

    fn initialize_population(&mut self) {
        self.population = (0..self.population_size)
            .into_iter()
            // .map(|_| self.create_individual())
            .map(|id| self.manager.build(id as u32))
            .collect();
    }

    fn generation(&mut self) {
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

    fn print_result(&self) {
        let mut buffer = String::new();

        buffer.push_str(&format!("Génération : {}\n", self.generation));

        for individual in self.population.iter() {
            buffer.push_str(&format!(" - {:?}\n", individual));
        }

        println!("{}", buffer);
    }
}
