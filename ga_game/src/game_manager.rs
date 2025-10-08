use ga_core::prelude::Individual;
use ga_ecs::prelude::{EntityId, EntityManager};
use ga_vec2::{FVec2, Normalized, Vec2};

use crate::{
    prelude::{
        Behaviour, Cercle, Enemy, Food, FromIndividual, Life, Position, RandomMovement, Speed,
        Survivor, SurvivorIndividual,
    },
    utils::collider,
};

pub trait GameManager<I: Individual> {
    fn generate_population(&mut self, population: &Vec<I>);
    fn update_cycle(&mut self);
    fn end_generation(&mut self, population: &mut Vec<I>);
}

impl GameManager<SurvivorIndividual> for EntityManager {
    fn generate_population(&mut self, population: &Vec<SurvivorIndividual>) {
        self.clear();

        for _ in 0..25 {
            spawn_enemy(self);
            spawn_food(self);
        }

        for individual in population {
            spawn_survivor(self, individual);
        }
    }

    fn update_cycle(&mut self) {
        update_foods(self);
        update_enemies(self);
        update_survivors(self);
    }

    fn end_generation(&mut self, population: &mut Vec<SurvivorIndividual>) {
        for (_, survivor) in self.get_components::<Survivor>() {
            let fitness = survivor.time_alive.elapsed().as_secs_f32();

            population
                .iter_mut()
                .find(|individual| individual.id == survivor.individual_id)
                .map(|individual| individual.fitness *= fitness);
        }
    }
}

fn spawn_survivor(manager: &mut EntityManager, individual: &SurvivorIndividual) -> EntityId {
    manager
        .build_entity()
        .with(Position(FVec2::ZERO))
        .with(Cercle::from_individual(individual))
        .with(Speed::from_individual(individual))
        .with(Behaviour::from_individual(individual))
        .with(Life::from_individual(individual))
        .with(Survivor::from_individual(individual))
        .id()
}

fn spawn_enemy(manager: &mut EntityManager) -> EntityId {
    manager
        .build_entity()
        .with(Position(FVec2::ZERO))
        .with(Cercle(1.0))
        .with(Speed(5.0))
        .with(RandomMovement {
            direction: Vec2::new(rand::random_range(-1.0..1.0), rand::random_range(-1.0..1.0)),
        })
        .with(Enemy { attack: 1 })
        .id()
}

fn spawn_food(manager: &mut EntityManager) -> EntityId {
    manager
        .build_entity()
        .with(Position(FVec2::ZERO))
        .with(Cercle(1.0))
        .with(Food(1))
        .id()
}

fn update_survivors(_manager: &mut EntityManager) {}

fn update_enemies(manager: &mut EntityManager) {
    for (id, enemy) in manager.get_components::<Enemy>() {
        let (Some(mut position), Some(cercle), Some(movement)) = (
            manager.entity_component_first_mut::<Position>(&id),
            manager.entity_component_first::<Cercle>(&id),
            manager.entity_component_first::<RandomMovement>(&id),
        ) else {
            continue;
        };

        position.0 += movement.direction.normalize_or_zero();

        for (survivor_id, _) in manager.get_components::<Survivor>() {
            let (Some(mut life), Some(survivor_position), Some(survivor_cercle)) = (
                manager.entity_component_first_mut::<Life>(&survivor_id),
                manager.entity_component_first::<Position>(&survivor_id),
                manager.entity_component_first::<Cercle>(&survivor_id),
            ) else {
                continue;
            };

            if collider::is_collision(&position, &cercle, &survivor_position, &survivor_cercle) {
                life.0 -= enemy.attack;
            }
        }
    }
}

fn update_foods(manager: &mut EntityManager) {
    for (food_id, food) in manager.get_components::<Food>() {
        let f_position = manager
            .entity_component_first::<Position>(&food_id)
            .unwrap();
        let f_cercle = manager.entity_component_first::<Cercle>(&food_id).unwrap();

        for (survivor_id, _) in manager.get_components::<Survivor>() {
            let (Some(mut life), Some(survivor_position), Some(survivor_cercle)) = (
                manager.entity_component_first_mut::<Life>(&survivor_id),
                manager.entity_component_first::<Position>(&survivor_id),
                manager.entity_component_first::<Cercle>(&survivor_id),
            ) else {
                continue;
            };

            if collider::is_collision(&f_position, &f_cercle, &survivor_position, &survivor_cercle)
            {
                life.eat(&food);
            }
        }
    }
}

pub fn draw(manager: &mut EntityManager) {
    let height = 50i32;
    let width = height;

    let mut buffer = vec!['.'; (width * height) as usize];

    for (_, position) in manager.get_components::<Position>() {
        // let Some(cercle) = self.entity_component_first::<Cercle>(&id) else {
        //     continue;
        // };

        let px = (position.0.x + width as f32 / 2.0) as i32;
        let py = (position.0.y + height as f32 / 2.0) as i32;
        let index = (py * width + px) as usize;

        if index < buffer.len() && px >= 0 && px < width && py >= 0 && py < height {
            buffer[index] = 'X';
        }
    }

    for y in 0..height {
        for x in 0..width {
            print!("{} ", buffer[(y * width + x) as usize]);
        }
        println!();
    }
}
