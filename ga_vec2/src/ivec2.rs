use crate::{FVec2, Normalized, Vec2};

pub type IVec2 = Vec2<i32>;

impl IVec2 {
    pub const ZERO: Self = Self { x: 0, y: 0 };
}

impl Normalized for IVec2 {
    type Output = FVec2;

    fn length_squared(&self) -> f32 {
        (self.x * self.x + self.y + self.y) as f32
    }

    fn normalize_or(&self, default: Self::Output) -> Self::Output {
        match self.length() {
            0.0 => default,
            magnetude => Vec2::new(self.x as f32 / magnetude, self.y as f32 / magnetude),
        }
    }

    fn normalize_or_zero(&self) -> Self::Output {
        self.normalize_or(Self::Output::ZERO)
    }
}
