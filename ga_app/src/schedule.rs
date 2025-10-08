use std::{
    any::{Any, TypeId},
    hash::Hash,
};

pub trait ScheduleLabel: 'static {}

#[derive(Hash, PartialEq, Eq)]
pub(crate) struct ScheduleKey(TypeId);

impl<T: ScheduleLabel> From<T> for ScheduleKey {
    fn from(value: T) -> Self {
        Self(value.type_id())
    }
}

pub struct Startup;
pub struct Update;

impl ScheduleLabel for Startup {}
impl ScheduleLabel for Update {}
