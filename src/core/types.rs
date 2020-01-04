#[allow(dead_code)]
#[allow(non_snake_case)]

use std::fmt;
use std::f32;
use std::fmt::Debug;


pub trait Spatial2D {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn pos(&self) -> (f32, f32) {
        (self.x(), self.y())
    }
    fn distance_from(&self, other: &dyn Spatial2D) -> f32 {
        let x = self.x() - other.x();
        let y = self.y() - other.y();
        ((x*x) + (y*y)).sqrt()
    }
}

pub trait Spatial3D {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;
    fn loc(&self) -> (f32, f32, f32) {
        (self.x(), self.y(), self.z())
    }
    fn distance_from(&self, other: &dyn Spatial3D) -> f32 {
        let x = self.x() - other.x();
        let y = self.y() - other.y();
        let z = self.z() - other.z();
        ((x*x) + (y*y) + (z*z)).sqrt()
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
    pub x: f32,
    pub y: f32,
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

impl Spatial2D for [f32; 2] {
    fn x(&self) -> f32 {self[0]}
    fn y(&self) -> f32 {self[1]}
}

impl Spatial2D for [f64; 2] {
    fn x(&self) -> f32 {self[0] as f32}
    fn y(&self) -> f32 {self[1] as f32}
}

impl Spatial3D for [f32; 3] {
    fn x(&self) -> f32 {self[0]}
    fn y(&self) -> f32 {self[1]}
    fn z(&self) -> f32 {self[2]}
}

impl Spatial3D for [f64; 3] {
    fn x(&self) -> f32 {self[0] as f32}
    fn y(&self) -> f32 {self[1] as f32}
    fn z(&self) -> f32 {self[2] as f32}
}

impl Debug for Spatial2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {:?}, y: {:?}", self.x(), self.y())
    }
}




