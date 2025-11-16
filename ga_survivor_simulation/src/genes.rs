use ga_core::{Gene, GeneRange};

pub enum SurvivorGene {
    Behaviour,
    Health,
    Speed,
    Size,
}

impl Gene for SurvivorGene {
    fn global_range(&self) -> GeneRange {
        match self {
            SurvivorGene::Behaviour => GeneRange::new(0.0, 2.0),
            SurvivorGene::Health => GeneRange::new(1.0, 5.0),
            SurvivorGene::Speed => GeneRange::new(0.0, 2.0),
            SurvivorGene::Size => GeneRange::new(0.1, 2.0),
        }
    }

    fn mutation_range(&self) -> GeneRange {
        match self {
            SurvivorGene::Behaviour => GeneRange::new(-1.0, 1.0),
            SurvivorGene::Health => GeneRange::new(-1.0, 1.0),
            SurvivorGene::Speed => GeneRange::new(-0.2, 0.2),
            SurvivorGene::Size => GeneRange::new(-0.1, 0.1),
        }
    }

    fn mutation_rate(&self) -> f64 {
        match self {
            SurvivorGene::Behaviour => 1.0 / 100.0,
            SurvivorGene::Health => 1.0 / 100.0,
            SurvivorGene::Speed => 1.0 / 100.0,
            SurvivorGene::Size => 1.0 / 100.0,
        }
    }
}
