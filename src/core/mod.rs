mod bounds;
mod error;
mod quadrant;
mod types;

pub use error::{SpatialError, Result};
pub use quadrant::{Quadrant, QUADRANTS};
pub use types::*;
pub use bounds::Bounds;