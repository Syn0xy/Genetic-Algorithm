mod components;

pub mod genes;
pub mod manager;
pub mod survivor;

pub mod prelude {
    pub use crate::genes::*;
    pub use crate::manager::*;
    pub use crate::survivor::*;
}
