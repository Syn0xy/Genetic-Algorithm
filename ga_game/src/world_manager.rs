use std::{collections::HashMap, fmt::Debug};

use ga_core::{Individual, IndividualId, IndividualManager, Simulation};
use ga_ecs::prelude::{EntityId, EntityManager, IntoEntity};

use crate::{
    debug,
    entity::{EnemyEntity, FoodEntity},
    update,
};

#[derive(Default)]
pub struct WorldManager<I: Individual, IM: IndividualManager<I>> {
    entity_manager: EntityManager,
    simulation: Simulation<I, IM>,
    individual_entities: HashMap<EntityId, IndividualId>,
}

impl<I, IM> WorldManager<I, IM>
where
    I: Individual + IntoEntity,
    IM: IndividualManager<I>,
{
    pub fn generate(&mut self) {
        self.simulation.generation();
        self.individual_entities.clear();
        self.entity_manager.clear();

        for _ in 0..20 {
            self.entity_manager.insert_entity(&EnemyEntity);
            self.entity_manager.insert_entity(&FoodEntity);
        }

        for individual in self.simulation.population() {
            let entity_id = self.entity_manager.insert_entity(individual);

            self.individual_entities.insert(entity_id, *individual.id());
        }
    }
}

impl<I, IM> WorldManager<I, IM>
where
    I: Individual,
    IM: IndividualManager<I>,
{
    pub fn new(manager: IM) -> Self {
        Self {
            entity_manager: Default::default(),
            simulation: Simulation::new(manager),
            individual_entities: Default::default(),
        }
    }

    pub fn initialize(&mut self, population_size: usize) {
        self.simulation.initialize_population(population_size);
    }

    pub fn update(&mut self) {
        update::update_foods(&mut self.entity_manager);
        update::update_enemies(&mut self.entity_manager);
        update::update_survivors(&mut self.entity_manager);
    }

    pub fn display_world(&self) {
        println!("\x1B[2J\x1B[1;1H");
        debug::draw(&self.entity_manager);
    }
}

impl<I, IM> WorldManager<I, IM>
where
    I: Individual + Debug,
    IM: IndividualManager<I>,
{
    pub fn display_simulations(&self) {
        self.simulation.print_result();
    }
}
