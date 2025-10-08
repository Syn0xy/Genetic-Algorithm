use std::collections::HashMap;

use crate::{
    prelude::{ScheduleKey, ScheduleLabel},
    system::System,
};

#[derive(Default)]
pub struct App {
    systems: HashMap<ScheduleKey, Vec<Box<dyn System>>>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_systems<T, S, I>(&mut self, schedule: T, systems: I)
    where
        T: ScheduleLabel,
        S: System + 'static,
        I: IntoIterator<Item = S>,
    {
        let s = self.systems.entry(schedule.into()).or_default();

        systems
            .into_iter()
            .for_each(|system| s.push(Box::new(system)));
    }
}
