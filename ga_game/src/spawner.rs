use ga_ecs::prelude::{EntityCommands, EntityId, IntoEntity};
use ga_vec2::{FVec2, Randomized, Vec2};

use crate::prelude::{Cercle, Enemy, Food, Position, Speed};

pub struct EnemyEntity;

pub struct FoodEntity;

impl IntoEntity for EnemyEntity {
    fn build(&self, mut commands: EntityCommands<'_>) -> EntityId {
        commands
            .insert(Position(FVec2::ZERO))
            .insert(Cercle(1.0))
            .insert(Speed(5.0))
            .insert(Enemy {
                attack: 1,
                direction: Vec2::random_range(-1.0..=1.0),
            })
            .id()
    }
}

impl IntoEntity for FoodEntity {
    fn build(&self, mut commands: EntityCommands<'_>) -> EntityId {
        commands
            .insert(Position(FVec2::ZERO))
            .insert(Cercle(1.0))
            .insert(Food { amount: 1 })
            .id()
    }
}
