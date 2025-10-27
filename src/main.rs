use ga_core::prelude::{Individual, IndividualManager, SimulationHandler};
use ga_game::prelude::WorldManager;
use ga_survivor_simulation::prelude::SurvivorManager;

const POPULATION_SIZE: usize = 16;
const GENERATION_MAX: usize = 8;
const _CYCLE_COUNT: usize = 50;

fn main() {
    let mut world = WorldManager::new();

    world.add_simulation(SimulationHandler::new(
        SurvivorManager::new(),
        POPULATION_SIZE,
    ));

    world.add_simulation(SimulationHandler::new(FoodManager, 20));

    world.initialize();

    for _ in 0..GENERATION_MAX {
        world.generate();

        // let population = simulation.population_mut();

        // entity_manager.clear();
        // entity_manager.generate_population(&population);

        // for _ in 0..CYCLE_COUNT {
        //     entity_manager.update_cycle();
        //     // entity_manager.draw();
        //     // thread::sleep(Duration::from_secs_f32(0.05));
        // }

        world.end_generation();
    }

    world.display();
}

struct FoodManager;
struct FoodIndividual;

impl Individual for FoodIndividual {
    fn id(&self) -> &ga_core::prelude::IndividualId {
        &0
    }

    fn fitness(&self) -> f32 {
        0.0
    }
}

impl IndividualManager<FoodIndividual> for FoodManager {
    fn build(&mut self, id: ga_core::prelude::IndividualId) -> I {
        FoodIndividual
    }

    fn mutate(&self, _: &mut FoodIndividual) {}

    fn crossover(&self, _: &mut FoodIndividual, _: &mut FoodIndividual) {}

    fn calculate_fitness(&self, _: &mut FoodIndividual) -> f32 {
        0.0
    }

    fn select_parent(&self, _: &Vec<FoodIndividual>) -> Option<FoodIndividual> {
        Some(FoodIndividual)
    }
}
