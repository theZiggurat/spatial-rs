use crate::linear_quadtree::Key;
use crate::core::{Spatial2D, Bounds};
use hashbrown::HashMap;

enum QuadtreeEntry<S> {
    Branch,
    Leaf(S),
    Saturated(S)
}

#[derive(Default)]
pub struct LinearQuadtree<S> {
    map: HashMap<Key, QuadtreeEntry<S>>,
    space_boundary: Bounds,
}

impl<S> LinearQuadtree<S> {
    pub fn new(space_boundary: Bounds) -> Self {
        Self {
            map: HashMap::new(),
            space_boundary
        }
    }
}

impl<S> LinearQuadtree<S>
    where S: Spatial2D + Copy {

    pub fn insert(&mut self, loc: S) -> Key {
        let mut ret: Key = Default::default();
        let bounds = self.space_boundary;
        while {
            let bounds = bounds;
            let quad = bounds.find_quadrant(&loc);
            ret.write_level(quad);
            self.map.insert(ret, QuadtreeEntry::Leaf(loc))
                .is_some()
        } {}
        ret
    }

    pub fn neighboring_keys(&self, key: Key) -> Vec<Key> {
        let same_size_neighbors = key.compute_neighbors();
        for same_size_key
    }

    pub fn neighbors(&self, key: Key) -> Vec<&S> {

    }

    pub fn neighbors_mut(&mut self, key: Key) -> Vec<&mut S> {

    }


}