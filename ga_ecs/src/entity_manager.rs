use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
};

use crate::prelude::{EntityCommands, IntoEntity};

pub type EntityId = u32;

#[derive(Default)]
pub struct EntityManager {
    entity_id_sequence: EntityId,
    components: HashMap<(EntityId, TypeId), Vec<RefCell<Box<dyn Any>>>>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.entity_id_sequence = EntityId::default();
        self.components.clear();
    }

    pub fn create_entity(&mut self) -> EntityId {
        let entity_id = self.entity_id_sequence;
        self.entity_id_sequence += 1;
        entity_id
    }

    pub fn build_entity(&mut self) -> EntityCommands<'_> {
        EntityCommands {
            entity_id: self.create_entity(),
            manager: self,
        }
    }

    pub fn entity(&mut self, entity_id: EntityId) -> EntityCommands<'_> {
        EntityCommands {
            entity_id,
            manager: self,
        }
    }

    pub fn insert_entity<IE: IntoEntity>(&mut self, into_entity: &IE) -> EntityId {
        into_entity.build(self.build_entity())
    }

    pub fn destroy(&mut self, entity_id: EntityId) {
        self.components.retain(|(id, _), _| !entity_id.eq(id));
    }

    pub fn attach_component<T: 'static>(&mut self, entity_id: EntityId, component: T) {
        self.components
            .entry((entity_id, component.type_id()))
            .or_default()
            .push(RefCell::new(Box::new(component)));
    }

    pub fn get_entities<T: 'static>(&self) -> impl Iterator<Item = &EntityId> {
        self.components
            .keys()
            .filter_map(|(id, type_id)| type_id.eq(&TypeId::of::<T>()).then_some(id))
    }

    pub fn components<T: 'static>(&self) -> impl Iterator<Item = (EntityId, Ref<'_, T>)> {
        self.components
            .iter()
            .filter(|((_, type_id), _)| type_id == &TypeId::of::<T>())
            .flat_map(|((id, _), cells)| {
                cells.iter().filter_map(|cell| {
                    Ref::filter_map(cell.borrow(), |boxed| boxed.downcast_ref::<T>())
                        .ok()
                        .map(|component| (*id, component))
                })
            })
    }

    pub fn components_mut<T: 'static>(&self) -> impl Iterator<Item = (EntityId, RefMut<'_, T>)> {
        self.components
            .iter()
            .filter(|((_, type_id), _)| type_id == &TypeId::of::<T>())
            .flat_map(|((id, _), cells)| {
                cells.iter().filter_map(|cell| {
                    RefMut::filter_map(cell.borrow_mut(), |boxed| boxed.downcast_mut::<T>())
                        .ok()
                        .map(|component| (*id, component))
                })
            })
    }

    pub fn get<T: 'static>(&self, entity_id: EntityId) -> impl Iterator<Item = Ref<'_, T>> {
        self.components
            .get(&(entity_id, TypeId::of::<T>()))
            .into_iter()
            .flat_map(|cells| cells.iter())
            .filter_map(|cell| {
                Ref::filter_map(cell.borrow(), |boxed| boxed.downcast_ref::<T>()).ok()
            })
    }

    pub fn get_first<T: 'static>(&self, entity_id: EntityId) -> Option<Ref<'_, T>> {
        self.get(entity_id).next()
    }

    pub fn get_mut<T: 'static>(&self, entity_id: EntityId) -> impl Iterator<Item = RefMut<'_, T>> {
        self.components
            .get(&(entity_id, TypeId::of::<T>()))
            .into_iter()
            .flat_map(|cells| cells.iter())
            .filter_map(|cell| {
                RefMut::filter_map(cell.borrow_mut(), |boxed| boxed.downcast_mut::<T>()).ok()
            })
    }

    pub fn get_first_mut<T: 'static>(&self, entity_id: EntityId) -> Option<RefMut<'_, T>> {
        self.get_mut(entity_id).next()
    }
}
