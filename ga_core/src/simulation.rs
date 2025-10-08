use std::fmt::Debug;

use crate::individual_manager::IndividualManager;

pub struct Simulation<IM: IndividualManager> {
    manager: IM,
    generation: u32,
    population: Vec<IM::IndType>,
}

impl<IM: IndividualManager> Simulation<IM>
where
    IM::IndType: Debug,
{
    pub fn new(manager: IM) -> Self {
        Self {
            generation: 0,
            manager,
            population: Vec::default(),
        }
    }

    pub fn population_mut(&mut self) -> &mut Vec<IM::IndType> {
        &mut self.population
    }

    pub fn init(&mut self, population_size: usize) {
        self.population = (0..population_size)
            .into_iter()
            .map(|_| self.manager.build())
            .collect();
    }

    pub fn display(&self) {
        let mut buffer = String::new();

        buffer.push_str(&format!("Génération : {}\n", self.generation));

        for individual in self.population.iter() {
            buffer.push_str(&format!("- {:?}\n", individual));
        }

        println!("{}", buffer);
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
