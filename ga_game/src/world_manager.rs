use std::collections::HashMap;

use ga_core::prelude::{Individual, IndividualId, Simulation};
use ga_ecs::prelude::{EntityId, EntityManager, IntoEntity};

use crate::spawner::{EnemyEntity, FoodEntity};

#[derive(Default)]
pub struct WorldManager<I: Individual + IntoEntity> {
    entity_manager: EntityManager,
    simulations: Vec<Box<dyn Simulation<I>>>,
    individual_entities: HashMap<EntityId, IndividualId>,
}

impl<I: Individual + IntoEntity + Default> WorldManager<I> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_simulation<T: Simulation<I> + 'static>(&mut self, simulation: T) {
        self.simulations.push(Box::new(simulation));
    }

    pub fn initialize(&mut self) {
        for s in &mut self.simulations {
            s.initialize_population();
        }
    }

    pub fn generate(&mut self) {
        self.individual_entities.clear();
        self.entity_manager.clear();

        for _ in 0..20 {
            self.entity_manager.insert_entity(&EnemyEntity);
            self.entity_manager.insert_entity(&FoodEntity);
        }

        for s in &self.simulations {
            for individual in s.population() {
                let entity_id = self.entity_manager.insert_entity(individual);

                self.individual_entities.insert(entity_id, *individual.id());
            }
        }
    }

    pub fn end_generation(&mut self) {
        // for (_, survivor) in self.entity_manager.components::<Survivor>() {
        //     let fitness = survivor.time_alive.elapsed().as_secs_f32();

        //     population
        //         .iter_mut()
        //         .find(|individual| individual.id == survivor.individual_id)
        //         .map(|individual| individual.fitness *= fitness);
        // }
    }

    pub fn display(&self) {
        self.simulations.iter().for_each(|s| s.print_result());
    }
}
