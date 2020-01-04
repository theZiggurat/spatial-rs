
use std::fmt;
use std::marker::PhantomData;
use slotmap::{SlotMap, DefaultKey, Values, ValuesMut};
use crate::core::Spatial2D;
use crate::quadrant::Quadrant;

const MAX_RECURCION: u32 = 8;

#[derive(Debug)]
pub struct Quadtree<T>
    where T: Copy {
    container: SlotMap<DefaultKey, T>,
    root: QuadtreeNode<T>,
    pub bounds: Bounds
}

impl<T> Quadtree<T>
    where T: Copy {
    pub fn new(bounds: Bounds) -> Self {
        Quadtree {
            container: SlotMap::new(),
            root: QuadtreeNode::Empty,
            bounds
        }
    }
}

impl<T> Quadtree<T>
    where T: Spatial2D + Copy + PartialEq {

    pub fn try_insert(&mut self, data: T) {

        if !self.bounds.is_point_within(&data) {
            return;
        }

        let key = self.container.insert(data);
        self.root.insert(key, self.bounds, &self.container, 0);
    }

    pub fn insert(&mut self, data: T) {
        self.try_insert(data);
    }

    pub fn contains(&self, p: T) -> bool {
        if !self.bounds.is_point_within(&p) {
            false
        } else {
            self.root.contains(p, &self.container, self.bounds)
        }
    }

    pub fn bounds(&self) -> Vec<Bounds> {
        let mut vec = vec![];
        self.root.bounds(&mut vec, self.bounds);
        vec
    }

    pub fn bounds_with_type(&self) -> Vec<(Bounds, BoundType)> {
        let mut vec = vec![];
        self.root.bounds_with_type(&mut vec, self.bounds);
        vec
    }

    pub fn rebuild_tree(&mut self) {
        self.root = QuadtreeNode::Empty;
        for key in self.container.keys() {
            self.root.insert(key, self.bounds, &self.container, 0);
        }
    }

    pub fn remove(&mut self, p: T) {
        if let Some(key) = self.root.remove(p, &self.container, self.bounds) {
            self.container.remove(key);
        }
    }

    pub fn within(&self, p: &dyn Spatial2D, radius: f32) -> Vec<T> {

        let mut vec = vec![];
        let mut enclosing_bound = Bounds::new(
            p.x() - radius, p.x() + radius, p.y() - radius, p.y() + radius
        );
        if !enclosing_bound.overlaps(self.bounds) {return vec;}
        enclosing_bound.truncate(self.bounds);

        vec

    }

    pub fn closest(&self, p: T) -> Option<T> {
        None
    }

    pub fn neighbors(&self, p: T) -> Vec<T> {
        vec![]
    }

    pub fn values(&self) -> Values<DefaultKey, T> {
        self.container.values()
    }

    pub fn values_mut(&mut self) -> ValuesMut<DefaultKey, T> {
        self.container.values_mut()
    }
}

#[derive(Debug, PartialEq)]
pub enum QuadtreeNode<T> {
    Saturated(Vec<DefaultKey>),
    Branch(Branch<T>),
    Leaf(DefaultKey),
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
        key: DefaultKey,
        bounds: Bounds,
        container: &SlotMap<DefaultKey, T>,
        r_lvl: u32
        )  {

        match self {
            QuadtreeNode::Saturated(vec) => {
                vec.push(key);
            }
            QuadtreeNode::Branch(_) => {
                let quadrant = bounds.find_quadrant(&container[key]);
                self.insert_in_branch(key, quadrant, bounds.sub_bound(quadrant), container, r_lvl);
            }
            QuadtreeNode::Leaf(other_key) => {

                let other_key = *other_key;
                if r_lvl == MAX_RECURCION {
                    *self = QuadtreeNode::Saturated(vec![key, other_key]);
                } else {
                    *self = QuadtreeNode::new_branch();

                    let (q1, q2) = {
                        let p1 = &container[key] as &Spatial2D;
                        let p2 = &container[other_key] as &Spatial2D;

                        let q1 = bounds.find_quadrant(p1);
                        let q2 = bounds.find_quadrant(p2);

                        //println!("P1: {:?} in {:?} | P2: {:?} in {:?}", p1, q1, p2, q2);

                        (q1, q2)
                    };




                    if q1 == q2 {
                        let new_bound = bounds.sub_bound(q1);
                        self.insert_in_branch(key, q1,new_bound, container, r_lvl);
                        self.insert_in_branch(other_key, q1,new_bound, container, r_lvl);
                    } else {
                        let new_bound = bounds.sub_bound(q1);
                        self.insert_in_branch(key, q1,new_bound, container, r_lvl);
                        let new_bound = bounds.sub_bound(q2);
                        self.insert_in_branch(other_key, q2,new_bound, container, r_lvl);
                    }
                }
            }
            QuadtreeNode::Empty => {
                *self = QuadtreeNode::Leaf(key);
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
        key: DefaultKey,
        quadrant: Quadrant,
        bounds: Bounds,
        container: &SlotMap<DefaultKey, T>,
        r_lvl: u32
    ) {
        if let QuadtreeNode::Branch(branch) = self {
            match quadrant {
                Quadrant::TL => branch.TL.insert(key, bounds, container, r_lvl+1),
                Quadrant::TR => branch.TR.insert(key, bounds, container, r_lvl+1),
                Quadrant::BL => branch.BL.insert(key, bounds, container, r_lvl+1),
                Quadrant::BR => branch.BR.insert(key, bounds, container, r_lvl+1)
            }
        }
    }

    fn bounds(&self, vec: &mut Vec<Bounds>, curr_bound: Bounds) {
        match self {
            QuadtreeNode::Saturated(_) => {
                vec.push(curr_bound);
            }
            QuadtreeNode::Branch(branch) => {
                vec.push(curr_bound);
                branch.TL.bounds(vec, curr_bound.sub_bound(Quadrant::TL));
                branch.TR.bounds(vec, curr_bound.sub_bound(Quadrant::TR));
                branch.BL.bounds(vec, curr_bound.sub_bound(Quadrant::BL));
                branch.BR.bounds(vec, curr_bound.sub_bound(Quadrant::BR));
            },
            QuadtreeNode::Leaf(_) => {
                vec.push(curr_bound);
            },
            QuadtreeNode::Empty => ()
        }

    }

    fn bounds_with_type(
        &self,
        vec: &mut Vec<(Bounds, BoundType)>,
        curr_bound: Bounds
    ) {
        match self {
            QuadtreeNode::Saturated(_) => {
                vec.push((curr_bound, BoundType::Saturated));
            }
            QuadtreeNode::Branch(branch) => {
                vec.push((curr_bound, BoundType::Branch));
                branch.TL.bounds_with_type(vec, curr_bound.sub_bound(Quadrant::TL));
                branch.TR.bounds_with_type(vec, curr_bound.sub_bound(Quadrant::TR));
                branch.BL.bounds_with_type(vec, curr_bound.sub_bound(Quadrant::BL));
                branch.BR.bounds_with_type(vec, curr_bound.sub_bound(Quadrant::BR));
            },
            QuadtreeNode::Leaf(_) => {
                vec.push((curr_bound, BoundType::Leaf));
            },
            QuadtreeNode::Empty => ()
        }

    }

    fn contains(
        &self,
        p: T, container: &SlotMap<DefaultKey, T>,
        curr_bound: Bounds
    ) -> bool {
        match self {
            QuadtreeNode::Saturated(vec) => {
                vec.iter().any(|key| container[*key] == p)
            }
            QuadtreeNode::Branch(branch) => {
                let quadrant = curr_bound.find_quadrant(&p);
                match quadrant {
                    Quadrant::TL => branch.TL.contains(p, container, curr_bound.sub_bound(quadrant)),
                    Quadrant::TR => branch.TR.contains(p, container, curr_bound.sub_bound(quadrant)),
                    Quadrant::BL => branch.BL.contains(p, container, curr_bound.sub_bound(quadrant)),
                    Quadrant::BR => branch.BR.contains(p, container, curr_bound.sub_bound(quadrant)),
                }
            }
            QuadtreeNode::Leaf(key) => {
                if container[*key] == p {
                    true
                } else {
                    false
                }
            }
            QuadtreeNode::Empty => {
                false
            }
        }
    }

    fn remove(
        &mut self,
        p: T,
        container: &SlotMap<DefaultKey, T>,
        curr_bound: Bounds
    ) -> Option<DefaultKey> {
        match self {
            QuadtreeNode::Saturated(vec) => {
                for (idx, key) in vec.clone().iter().enumerate() {
                    if container[*key] == p {
                        vec.remove(idx);
                        if vec.len() == 1 {
                            *self = QuadtreeNode::Leaf(*vec.get(0).unwrap())
                        }
                        return Some(*key)
                    }
                }
                None
            }
            QuadtreeNode::Branch(branch) => {
                let quadrant = curr_bound.find_quadrant(&p);
                match quadrant {
                    Quadrant::TL => branch.TL.remove(p, container, curr_bound.sub_bound(quadrant)),
                    Quadrant::TR => branch.TR.remove(p, container, curr_bound.sub_bound(quadrant)),
                    Quadrant::BL => branch.BL.remove(p, container, curr_bound.sub_bound(quadrant)),
                    Quadrant::BR => branch.BR.remove(p, container, curr_bound.sub_bound(quadrant)),
                }
            }
            QuadtreeNode::Leaf(key) => {
                let key = key.clone();
                if container[key] == p {
                    *self = QuadtreeNode::Empty;
                    Some(key)
                } else {
                    None
                }
            }
            QuadtreeNode::Empty => None
        }
    }

    fn smallest_enclosing(&self, test_bound: Bounds, curr_bound: Bounds) -> Option<&QuadtreeNode<T>> {
        None
    }
}

#[derive(Debug)]
pub enum BoundType {
    Leaf,
    Saturated,
    Branch
}





