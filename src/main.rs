use std::{thread, time::Duration};

use ga_game::WorldManager;
use ga_survivor_simulation::SurvivorManager;

const POPULATION_SIZE: usize = 16;
const GENERATION_MAX: usize = 16;
const CYCLE_COUNT: usize = 100;

fn main() {
    let mut world = WorldManager::new(SurvivorManager);

    world.initialize(POPULATION_SIZE);

    for _ in 0..GENERATION_MAX {
        world.generate();

        for _ in 0..CYCLE_COUNT {
            world.update();
            world.display_world();
            thread::sleep(Duration::from_secs_f32(0.2));
        }
    }

    world.display_simulations();
}
