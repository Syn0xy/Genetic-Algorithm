#[derive(Debug, Clone, Copy)]
pub struct GeneRange {
    min: f32,
    max: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct GeneSpec {
    pub name: &'static str,
    pub init_range: GeneRange,
    pub mut_range: GeneRange,
    pub mutation_rate: f32,
}

impl GeneSpec {
    pub const fn clamp(&self, v: f32) -> f32 {
        self.init_range.clamp(v)
    }
}

impl GeneRange {
    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub const fn clamp(&self, v: f32) -> f32 {
        v.clamp(self.min, self.max)
    }

    pub fn random_value(&self) -> f32 {
        rand::random_range(self.min..self.max)
    }
}

pub trait Gene {
    fn init_range(&self) -> GeneRange;
    fn mut_range(&self) -> GeneRange;
    fn mutation_rate(&self) -> f32;
}
