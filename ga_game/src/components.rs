use std::time::Instant;

use ga_vec2::FVec2;

#[derive(Debug)]
pub struct Position(pub FVec2);

#[derive(Debug)]
pub struct Cercle(pub f32);

#[derive(Debug)]
pub struct Speed(pub f32);

#[derive(Debug)]
pub struct Life(pub u8);

#[derive(Debug)]
pub enum Behaviour {
    Fearful,
    Hungry,
    Random,
}

#[derive(Debug)]
pub struct Enemy {
    pub attack: u8,
    pub direction: FVec2,
}

#[derive(Debug)]
pub struct Food {
    pub amount: u8,
}

#[derive(Debug)]
pub struct Survivor {
    pub time_alive: Instant,
}
