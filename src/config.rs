use ga_core::prelude::{GeneRange, GeneSpec};

pub const GENERATION_MAX: usize = 8;
pub const POPULATION_SIZE: usize = 16;
pub const TOURNAMENT_SIZE: usize = 4; // sqrt(POPULATION_SIZE)
pub const CYCLE_COUNT: usize = 50;
pub const GENOME_SPECS: &[GeneSpec] = &[
    GeneSpec {
        name: "Behaviour",
        init_range: GeneRange::new(0.0, 2.0),
        mut_range: GeneRange::new(-1.0, 1.0),
        mutation_rate: 1.0 / 100.0,
    },
    GeneSpec {
        name: "Life",
        init_range: GeneRange::new(1.0, 5.0),
        mut_range: GeneRange::new(-1.0, 1.0),
        mutation_rate: 1.0 / 100.0,
    },
    GeneSpec {
        name: "Speed",
        init_range: GeneRange::new(0.0, 2.0),
        mut_range: GeneRange::new(-0.2, 0.2),
        mutation_rate: 1.0 / 100.0,
    },
    GeneSpec {
        name: "Size",
        init_range: GeneRange::new(0.1, 2.0),
        mut_range: GeneRange::new(-0.1, 0.1),
        mutation_rate: 1.0 / 100.0,
    },
];
