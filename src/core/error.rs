use std::fmt;
use std::error;

#[derive(Debug)]
pub enum SpatialError {
    QuadtreeInsertError
}

pub type Result<T> = std::result::Result<T, SpatialError>;

impl fmt::Display for SpatialError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "quad tree insert errror")
    }
}

impl error::Error for SpatialError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

