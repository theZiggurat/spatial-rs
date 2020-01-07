pub const QUADRANTS: [Quadrant; 4] = [Quadrant::BL, Quadrant::BR, Quadrant::TL, Quadrant::TR];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Quadrant {
    TL,
    TR,
    BL,
    BR,
}

//impl Quadrant {
//    /// Returns iterator of quadrants that is not self
//    pub fn all_others(&self) -> Vec<Quadrant> {
//        QUADRANTS.iter().cloned().filter(|q| q==self).collect()
//    }
//}

pub enum QuadtreeError {
    BoundsError,
    DepthError,
}

use std::fmt;
use std::iter::Filter;

impl fmt::Display for QuadtreeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QuadtreeError::BoundsError => write!(f, "Point out of bounds"),
            QuadtreeError::DepthError => write!(f, "Maximum tree recursion depth reached"),
        }

    }
}