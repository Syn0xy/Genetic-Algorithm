use ga_core::prelude::{Gene, GeneRange};

pub struct HealthGene;
pub struct BehaviourGene;
pub struct SpeedGene;
pub struct SizeGene;

impl Gene for BehaviourGene {
    fn global_range(&self) -> GeneRange {
        GeneRange::new(0.0, 2.0)
    }

    fn mutation_range(&self) -> GeneRange {
        GeneRange::new(-1.0, 1.0)
    }

    fn mutation_rate(&self) -> f64 {
        1.0 / 100.0
    }
}

impl Gene for HealthGene {
    fn global_range(&self) -> GeneRange {
        GeneRange::new(1.0, 5.0)
    }

    fn mutation_range(&self) -> GeneRange {
        GeneRange::new(-1.0, 1.0)
    }

    fn mutation_rate(&self) -> f64 {
        1.0 / 100.0
    }
}

impl Gene for SpeedGene {
    fn global_range(&self) -> GeneRange {
        GeneRange::new(0.0, 2.0)
    }

    fn mutation_range(&self) -> GeneRange {
        GeneRange::new(-0.2, 0.2)
    }

    fn mutation_rate(&self) -> f64 {
        1.0 / 100.0
    }
}

impl Gene for SizeGene {
    fn global_range(&self) -> GeneRange {
        GeneRange::new(0.1, 2.0)
    }

    fn mutation_range(&self) -> GeneRange {
        GeneRange::new(-0.1, 0.1)
    }

    fn mutation_rate(&self) -> f64 {
        1.0 / 100.0
    }
}
