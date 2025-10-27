use std::{
    any::{Any, TypeId},
    cell::{Ref, RefMut},
};

use crate::prelude::{EntityId, EntityManager};

pub struct EntityCommands<'em> {
    pub(crate) entity_id: EntityId,
    pub(crate) manager: &'em mut EntityManager,
}

impl<'em> EntityCommands<'em> {
    pub fn id(&self) -> EntityId {
        self.entity_id
    }

    pub fn insert<T: Any>(&mut self, component: T) -> &mut Self {
        self.manager.attach_component(self.entity_id, component);
        self
    }

    pub fn get<T: 'static>(&self) -> impl Iterator<Item = Ref<'_, T>> {
        self.manager
            .components
            .get(&(self.entity_id, TypeId::of::<T>()))
            .into_iter()
            .flat_map(|cells| cells.iter())
            .filter_map(|cell| {
                Ref::filter_map(cell.borrow(), |boxed| boxed.downcast_ref::<T>()).ok()
            })
    }

    pub fn get_first<T: 'static>(&self) -> Option<Ref<'_, T>> {
        self.get().next()
    }

    pub fn get_mut<T: 'static>(&self) -> impl Iterator<Item = RefMut<'_, T>> {
        self.manager
            .components
            .get(&(self.entity_id, TypeId::of::<T>()))
            .into_iter()
            .flat_map(|cells| cells.iter())
            .filter_map(|cell| {
                RefMut::filter_map(cell.borrow_mut(), |boxed| boxed.downcast_mut::<T>()).ok()
            })
    }

    pub fn get_first_mut<T: 'static>(&self) -> Option<RefMut<'_, T>> {
        self.get_mut().next()
    }
}
