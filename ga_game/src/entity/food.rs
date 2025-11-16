use ga_ecs::prelude::{EntityCommands, EntityId, IntoEntity};
use ga_vec2::FVec2;

use crate::{Cercle, Food, Position, Renderable};

pub struct FoodEntity;

impl IntoEntity for FoodEntity {
    fn build(&self, mut commands: EntityCommands<'_>) -> EntityId {
        commands
            .insert(Renderable('🧀'))
            .insert(Position(FVec2 {
                x: rand::random_range(-40.0..=40.0),
                y: rand::random_range(-20.0..=20.0),
            }))
            .insert(Cercle(1.0))
            .insert(Food { amount: 1 })
            .id()
    }
}
