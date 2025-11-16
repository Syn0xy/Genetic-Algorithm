use crate::{Normalized, Vec2};

pub type FVec2 = Vec2<f32>;

impl FVec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
}

impl Normalized for FVec2 {
    type Output = Self;

    fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    fn normalize_or(&self, default: Self) -> Self {
        match self.length() {
            0.0 => default,
            magnetude => Vec2::new(self.x / magnetude, self.y / magnetude),
        }
    }

    fn normalize_or_zero(&self) -> Self::Output {
        self.normalize_or(Self::Output::ZERO)
    }
}
