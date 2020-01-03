#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Quadrant {
    TL,
    TR,
    BL,
    BR,
}

pub enum QuadtreeError {
    BoundsError,
    DepthError,
}

use std::fmt;
impl fmt::Display for QuadtreeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QuadtreeError::BoundsError => write!(f, "Point out of bounds"),
            QuadtreeError::DepthError => write!(f, "Maximum tree recursion depth reached"),
        }

    }
}