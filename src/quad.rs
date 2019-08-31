use std::fmt;
use std::marker::PhantomData;
use crate::spatial::Spatial2D;

#[derive(Debug)]
pub struct Quadtree<T> {
    pub container: Vec<T>,
    pub root: QuadtreeNode<T>,
    pub bounds: Bounds
}

impl<T> Quadtree<T> {
    pub fn new(bounds: Bounds) -> Self {
        Quadtree {
            container: vec![],
            root: QuadtreeNode::Empty,
            bounds
        }
    }
}

impl<T> Quadtree<T>
    where T: Spatial2D + Copy + PartialEq {

    pub fn try_insert(&mut self, data: T){

        if !self.bounds.within(&data) {
            return;
        }

        let idx = self.container.len();
        self.container.push(data);

        self.root.insert(idx, self.bounds, &self.container);

    }

    pub fn insert(&mut self, data: T) {
        self.try_insert(data);
    }
}

#[derive(Debug, PartialEq)]
pub enum QuadtreeNode<T> {
    Branch(Branch<T>),
    Leaf(usize),
    Empty,
}

#[derive(Debug, PartialEq)]
pub struct Branch<T> {
    TL: Box<QuadtreeNode<T>>,
    TR: Box<QuadtreeNode<T>>,
    BL: Box<QuadtreeNode<T>>,
    BR: Box<QuadtreeNode<T>>,
    _phantom_data: PhantomData<T>
}

impl<T> QuadtreeNode<T>
    where T: Spatial2D + Copy + PartialEq
{
    pub fn insert(
        &mut self,
        idx: usize,
        bounds: Bounds,
        container: &Vec<T>
    )  {

        match self {
            QuadtreeNode::Branch(_) => {
                let quadrant = bounds.find_quadrant(container.get(idx).unwrap());
                self.insert_in_branch(idx, quadrant, bounds.sub_bound(quadrant), container);
            },
            QuadtreeNode::Leaf(other_idx) => {

                let other_idx = *other_idx;
                *self = QuadtreeNode::new_branch();
                let p1 = container.get(idx).unwrap() as &Spatial2D;
                let p2 = container.get(other_idx).unwrap() as &Spatial2D;

                let q1 = bounds.find_quadrant(p1);
                let q2 = bounds.find_quadrant(p2);

                if q1 == q2 {
                    let new_bound = bounds.sub_bound(q1);
                    self.insert_in_branch(idx, q1,new_bound, container);
                    self.insert_in_branch(other_idx, q1,new_bound, container);
                } else {
                    let new_bound = bounds.sub_bound(q1);
                    self.insert_in_branch(idx, q1,new_bound, container);
                    let new_bound = bounds.sub_bound(q2);
                    self.insert_in_branch(other_idx, q2,new_bound, container);
                }
            },
            QuadtreeNode::Empty => {
                *self = QuadtreeNode::Leaf(idx);
            }
        }
    }

    pub fn new_branch() -> QuadtreeNode<T> {
        QuadtreeNode::Branch(
            Branch {
                TL: Box::new(QuadtreeNode::Empty),
                TR: Box::new(QuadtreeNode::Empty),
                BL: Box::new(QuadtreeNode::Empty),
                BR: Box::new(QuadtreeNode::Empty),
                _phantom_data: PhantomData
            }
        )
    }

    pub fn insert_in_branch(
        &mut self,
        idx: usize,
        quadrant: Quadrant,
        bounds: Bounds,
        container: &Vec<T>
    ) {
        if let QuadtreeNode::Branch(branch) = self {
            match quadrant {
                Quadrant::TL => branch.TL.insert(idx, bounds, container),
                Quadrant::TR => branch.TR.insert(idx, bounds, container),
                Quadrant::BL => branch.BL.insert(idx, bounds, container),
                Quadrant::BR => branch.BR.insert(idx, bounds, container)
            }
        }
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bounds {
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
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

    pub fn within(&self, point: &dyn Spatial2D) -> bool {
        let (x, y) = point.pos();
        x >= self.x_min && x <= self.x_max && y >= self.y_min && y <= self.y_max
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
}

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

impl fmt::Display for QuadtreeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QuadtreeError::BoundsError => write!(f, "Point out of bounds"),
            QuadtreeError::DepthError => write!(f, "Maximum tree recursion depth reached"),
        }

    }
}

