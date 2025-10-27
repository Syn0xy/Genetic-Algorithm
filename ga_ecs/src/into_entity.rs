use crate::prelude::{EntityCommands, EntityId};

pub trait IntoEntity {
    fn build(&self, commands: EntityCommands<'_>) -> EntityId;
}
