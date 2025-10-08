mod gene;
mod individual;
mod individual_manager;
mod simulation;

pub mod prelude {
    pub use crate::gene::*;
    pub use crate::individual::*;
    pub use crate::individual_manager::*;
    pub use crate::simulation::*;
}
