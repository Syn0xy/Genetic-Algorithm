use std::collections::HashSet;

use ga_ecs::prelude::EntityManager;
use ga_vec2::Normalized;

use crate::{
    utils::collider,
    {Cercle, Enemy, Food, Life, Position, Speed, Survivor},
};

pub fn update_foods(manager: &mut EntityManager) {
    let mut food_destroy = HashSet::new();

    for (id, food) in manager.components::<Food>() {
        let (Some(food_position), Some(food_cercle)) = (
            manager.get_first::<Position>(id),
            manager.get_first::<Cercle>(id),
        ) else {
            continue;
        };

        for (survivor_id, _) in manager.components_mut::<Survivor>() {
            let (Some(mut survivor_life), Some(survivor_position), Some(survivor_cercle)) = (
                manager.get_first_mut::<Life>(survivor_id),
                manager.get_first::<Position>(survivor_id),
                manager.get_first::<Cercle>(survivor_id),
            ) else {
                continue;
            };

            if collider::is_collision(
                &food_position,
                &food_cercle,
                &survivor_position,
                &survivor_cercle,
            ) {
                survivor_life.eat(&food);
                food_destroy.insert(id);
            }
        }
    }

    for food_id in food_destroy {
        manager.destroy(food_id);
    }
}

pub fn update_enemies(manager: &mut EntityManager) {
    let mut survivor_destroy = HashSet::new();

    for (id, mut enemy) in manager.components_mut::<Enemy>() {
        let (Some(mut position), Some(cercle)) = (
            manager.get_first_mut::<Position>(id),
            manager.get_first::<Cercle>(id),
        ) else {
            continue;
        };

        let enemy_p = &mut position.0;

        *enemy_p += enemy.direction.normalize_or_zero();

        if enemy_p.x.abs() > 40.0 {
            enemy.direction.x = -enemy.direction.x;
            enemy_p.x = enemy_p.x.clamp(-40.0, 40.0);
        }

        if enemy_p.y.abs() > 20.0 {
            enemy.direction.y = -enemy.direction.y;
            enemy_p.y = enemy_p.y.clamp(-20.0, 20.0);
        }

        for (survivor_id, _) in manager.components_mut::<Survivor>() {
            let (Some(mut survivor_life), Some(survivor_position), Some(survivor_cercle)) = (
                manager.get_first_mut::<Life>(survivor_id),
                manager.get_first::<Position>(survivor_id),
                manager.get_first::<Cercle>(survivor_id),
            ) else {
                continue;
            };

            if collider::is_collision(&position, &cercle, &survivor_position, &survivor_cercle) {
                survivor_life.take(enemy.attack);

                if survivor_life.0 == 0 {
                    survivor_destroy.insert(survivor_id);
                }
            }
        }
    }

    for survivor in survivor_destroy {
        manager.destroy(survivor);
    }
}

pub fn update_survivors(manager: &mut EntityManager) {
    for (id, _) in manager.components::<Survivor>() {
        let (Some(mut position), Some(speed)) = (
            manager.get_first_mut::<Position>(id),
            manager.get_first::<Speed>(id),
        ) else {
            continue;
        };

        if rand::random() {
            position.0.x += speed.0 * rand::random_range(-1.0..=1.0);
        } else {
            position.0.y += speed.0 * rand::random_range(-1.0..=1.0);
        }
    }
}
