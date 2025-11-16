use ga_ecs::prelude::{EntityCommands, EntityId, IntoEntity};
use ga_vec2::{FVec2, Vec2};

use crate::{Cercle, Enemy, Position, Renderable, Speed};

pub struct EnemyEntity;

impl IntoEntity for EnemyEntity {
    fn build(&self, mut commands: EntityCommands<'_>) -> EntityId {
        commands
            .insert(Renderable('🐈'))
            .insert(Position(FVec2 {
                x: rand::random_range(-40.0..=40.0),
                y: rand::random_range(-20.0..=20.0),
            }))
            .insert(Cercle(1.0))
            .insert(Speed(5.0))
            .insert(Enemy {
                attack: 1,
                direction: Vec2 {
                    x: rand::random_range(-1.0..=1.0),
                    y: rand::random_range(-1.0..=1.0),
                },
            })
            .id()
    }
}
