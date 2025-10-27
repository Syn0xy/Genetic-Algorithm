// use ga_ecs::prelude::EntityManager;
// use ga_vec2::Normalized;

// use crate::{
//     prelude::{Cercle, Enemy, Food, Position, Speed, Survivor},
//     utils::collider,
// };

// fn update_survivors(manager: &mut EntityManager) {
//     for (id, _) in manager.components::<Survivor>() {
//         let entity = manager.entity(id);

//         let (Some(mut position), Some(speed)) = (
//             entity.get_first_mut::<Position>(),
//             entity.get_first::<Speed>(),
//         ) else {
//             continue;
//         };

//         position.0.x += 1.0 * speed.0;
//         position.0.y += 0.0 * speed.0;
//     }
// }

// fn update_enemies(manager: &mut EntityManager) {
//     for (enemy_id, enemy) in manager.components::<Enemy>() {
//         let enemy_entity = manager.entity(enemy_id);

//         let (Some(mut position), Some(cercle)) = (
//             enemy_entity.get_first_mut::<Position>(),
//             enemy_entity.get_first::<Cercle>(),
//         ) else {
//             continue;
//         };

//         position.0 += enemy.direction.normalize_or_zero();

//         for (survivor_id, mut survivor) in manager.components_mut::<Survivor>() {
//             let survivor_entity = manager.entity(survivor_id);

//             let (Some(survivor_position), Some(survivor_cercle)) = (
//                 survivor_entity.get_first::<Position>(),
//                 survivor_entity.get_first::<Cercle>(),
//             ) else {
//                 continue;
//             };

//             if collider::is_collision(&position, &cercle, &survivor_position, &survivor_cercle) {
//                 survivor.life -= enemy.attack;
//             }
//         }
//     }
// }

// fn update_foods(manager: &mut EntityManager) {
//     for (food_id, food) in manager.components::<Food>() {
//         let food_entity = manager.entity(food_id);

//         let (Some(food_position), Some(food_cercle)) = (
//             food_entity.get_first::<Position>(),
//             food_entity.get_first::<Cercle>(),
//         ) else {
//             continue;
//         };

//         for (survivor_id, mut survivor) in manager.components_mut::<Survivor>() {
//             let survivor_entity = manager.entity(survivor_id);

//             let (Some(survivor_position), Some(survivor_cercle)) = (
//                 survivor_entity.get_first::<Position>(),
//                 survivor_entity.get_first::<Cercle>(),
//             ) else {
//                 continue;
//             };

//             if collider::is_collision(
//                 &food_position,
//                 &food_cercle,
//                 &survivor_position,
//                 &survivor_cercle,
//             ) {
//                 survivor.eat(&food);
//             }
//         }
//     }
// }
