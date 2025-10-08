use std::any::Any;

use crate::prelude::{EntityId, EntityManager};

pub struct EntityBuilder<'a> {
    pub(crate) entity_id: EntityId,
    pub(crate) manager: &'a mut EntityManager,
}

impl<'a> EntityBuilder<'a> {
    pub(crate) fn new(entity_id: EntityId, manager: &'a mut EntityManager) -> Self {
        Self { entity_id, manager }
    }

    pub fn with<T: Any>(self, component: T) -> Self {
        self.manager.attach_component(self.entity_id, component);
        self
    }

    pub fn id(self) -> EntityId {
        self.entity_id
    }
}
