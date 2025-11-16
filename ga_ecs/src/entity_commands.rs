use std::any::Any;

use crate::prelude::{EntityId, EntityManager};

pub struct EntityCommands<'c> {
    pub(crate) entity_id: EntityId,
    pub(crate) manager: &'c mut EntityManager,
}

impl<'c> EntityCommands<'c> {
    pub fn id(&self) -> EntityId {
        self.entity_id
    }

    pub fn insert<T: Any>(&mut self, component: T) -> &mut Self {
        self.manager.attach_component(self.entity_id, component);
        self
    }
}
