use std::{
    thread,
    time::{Duration, Instant},
};

use ga_core::prelude::Simulation;
use ga_ecs::prelude::EntityManager;
use ga_game::prelude::{GameManager, SurvivorManager};

use crate::config::{CYCLE_COUNT, GENERATION_MAX, GENOME_SPECS, POPULATION_SIZE, TOURNAMENT_SIZE};

mod config;

fn main() {
    let manager = SurvivorManager::new(TOURNAMENT_SIZE, GENOME_SPECS.to_vec());
    let mut simulation = Simulation::new(manager);
    let mut entity_manager = EntityManager::new();

    simulation.init(POPULATION_SIZE);

    let start = Instant::now();

    for _ in 0..GENERATION_MAX {
        simulation.generation();
        let population = simulation.population_mut();

        entity_manager.clear();
        entity_manager.generate_population(&population);

        for _ in 0..CYCLE_COUNT {
            entity_manager.update_cycle();
            entity_manager.draw();
            thread::sleep(Duration::from_secs_f32(0.05));
        }

        entity_manager.end_generation(population);
    }

    simulation.display();

    println!("Time elapsed : {:?}", 1.0 / start.elapsed().as_secs_f32());

    // App::new().add_systems(Startup, [|| {}]);
}
