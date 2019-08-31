#[allow(dead_code)]
#[allow(non_snake_case)]

use std::fmt;


pub trait Spatial2D {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn pos(&self) -> (f32, f32) {
        (self.x(), self.y())
    }
}

pub trait Spatial3D {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;
    fn loc(&self) -> (f32, f32, f32) {
        (self.x(), self.y(), self.z())
    }
}

pub struct SpatialError;

impl fmt::Display for SpatialError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Item inserted is outside tree bounds")
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Point2D {
    x: f32,
    y: f32,
}

impl Point2D {
    pub fn new(x: f32, y: f32) -> Point2D {
        Point2D { x, y }
    }
}

impl Spatial2D for Point2D {
    fn x(&self) -> f32 {self.x}
    fn y(&self) -> f32 {self.y}
}



