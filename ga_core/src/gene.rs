pub trait Gene {
    fn global_range(&self) -> GeneRange;
    fn mutation_range(&self) -> GeneRange;
    fn mutation_rate(&self) -> f64;

    fn clamp(&self, a: f32) -> f32 {
        self.global_range().clamp(a)
    }

    fn random_global(&self) -> f32 {
        self.global_range().random_value()
    }

    fn random_mutation(&self) -> f32 {
        self.mutation_range().random_value()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GeneRange {
    min: f32,
    max: f32,
}

impl GeneRange {
    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub const fn clamp(&self, a: f32) -> f32 {
        a.clamp(self.min, self.max)
    }

    pub fn random_value(&self) -> f32 {
        rand::random_range(self.min..=self.max)
    }
}
