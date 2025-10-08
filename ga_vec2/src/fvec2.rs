use std::ops::{Add, AddAssign};

use crate::{Normalized, Vec2};

pub type FVec2 = Vec2<f32>;

impl FVec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
}

impl Normalized for FVec2 {
    type Output = Self;

    fn magnetude(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    fn normalize_or(&self, default: Self) -> Self {
        let magnetude = self.magnetude();

        if magnetude == 0.0 {
            default
        } else {
            Vec2::new(self.x / magnetude, self.y / magnetude)
        }
    }

    fn normalize_or_zero(&self) -> Self {
        self.normalize_or(Self::ZERO)
    }
}

impl Add for FVec2 {
    type Output = FVec2;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for FVec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
