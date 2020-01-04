use crate::core::{Quadrant, Spatial2D};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bounds {
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
}

impl Bounds {
    pub fn new(x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> Bounds {
        Self {
            x_min, x_max, y_min, y_max,
        }
    }

    pub fn sub_bound(&self, quadrant: Quadrant) -> Bounds {
        let Bounds {x_min, x_max, y_min, y_max} = *self;
        let (half_x, half_y) = self.half_bounds();
        match quadrant {
            Quadrant::TL => Bounds::new(x_min, half_x, y_min, half_y),
            Quadrant::TR => Bounds::new(half_x, x_max, y_min, half_y),
            Quadrant::BL => Bounds::new(x_min, half_x, half_y, y_max),
            Quadrant::BR => Bounds::new(half_x, x_max, half_y, y_max),
        }
    }

    pub fn half_bounds(&self) -> (f32, f32) {
        let Bounds {x_min, x_max, y_min, y_max} = self;
        let (width, height) = (x_max - x_min, y_max - y_min);
        (x_min + (width / 2.), y_min + (height / 2.))
    }

    pub fn is_point_within(&self, point: &dyn Spatial2D) -> bool {
        let (x, y) = point.pos();
        x >= self.x_min && x <= self.x_max && y >= self.y_min && y <= self.y_max
    }

    fn is_bound_within(&self, other_bound: Bounds) -> bool {
        self.x_min <= other_bound.x_min &&
        self.x_max >= other_bound.x_max &&
        self.y_min <= other_bound.y_min &&
        self.y_max >= other_bound.y_max
    }

    pub fn find_quadrant(&self, point: &dyn Spatial2D) -> Quadrant {
        let (x, y) = point.pos();
        let (half_x, half_y) = self.half_bounds();
        if x <= half_x {
            if y <= half_y {
                Quadrant::TL
            } else {
                Quadrant::BL
            }
        } else {
            if y <= half_y {
                Quadrant::TR
            } else {
                Quadrant::BR
            }
        }
    }

    pub fn truncate(&mut self, other: Bounds) {
        if self.x_min < other.x_min {
            self.x_min = other.x_min;
        }
        if self.x_max > other.x_max {
            self.x_max = other.x_max;
        }
        if self.y_min < other.y_min {
            self.y_min = other.y_min;
        }
        if self.y_max > other.y_max {
            self.y_max = other.y_max;
        }
    }

    pub fn overlaps(&self, other: Bounds) -> bool {
        ((self.x_min > other.x_min && self.x_min < other.x_max) ||
        (self.x_max > other.x_min && self.x_max < other.x_max)) &&
        ((self.y_min > other.y_min && self.y_min < other.y_max) ||
        (self.y_max > other.y_min && self.y_max < other.y_max))
    }
}

impl Default for Bounds {
    fn default() -> Self {
        Bounds::new(0., 1., 0., 1.)
    }
}