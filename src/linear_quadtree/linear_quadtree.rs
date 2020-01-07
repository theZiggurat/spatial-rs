

use crate::linear_quadtree::Key;
use crate::core::{Spatial2D, Bounds, QUADRANTS};
use hashbrown::HashMap;
use slotmap::SlotMap;

new_key_type!{
    pub struct SpatialKey;
}

enum QuadtreeEntry<S> {
    Branch,
    Leaf(S),
}

#[derive(Default)]
pub struct LinearQuadtree<S> {
    spatial_map: HashMap<Key, QuadtreeEntry<S>>,
    key_map: SlotMap<SpatialKey, Key>,
    space_boundary: Bounds,
}

impl<S> LinearQuadtree<S> {
    pub fn new(space_boundary: Bounds) -> Self {
        Self {
            spatial_map: HashMap::new(),
            key_map: SlotMap::with_key(),
            space_boundary
        }
    }
}

impl<S> LinearQuadtree<S>
    where S: Spatial2D + Copy {

    /// Inserts Spatial2D into quadtree and returns
    /// a persistent key that indexes it
    pub fn insert(&mut self, loc: S) -> SpatialKey {

        let mut ret: Key = Default::default();
        let mut bounds = self.space_boundary;

        loop {
            let quad = bounds.find_quadrant(&loc);
            bounds = bounds.sub_bound(quad);
            println!("Bounds: {:?}", bounds);
            let mut child = match ret.child(quad) {
                Ok(child) => child,
                Err(e) => {
                    ret.increment_overflow();
                    ret
                }
            };
            match self.spatial_map.get(&child) {
                // hit a branch; keep going
                Some(QuadtreeEntry::Branch) => {
                    println!("Branch at {}", child);
                    ret = child
                },
                // solve collision by moving both colliding keys
                // down the tree
                Some(QuadtreeEntry::Leaf(loc2)) => {
                    println!("Collision detected");
                    let loc2 = loc2.clone();
                    let invalid_key = self.find_key_in_keymap(&child).unwrap();
                    let old_key = child.clone();
                    loop {
                        self.spatial_map.insert(child, QuadtreeEntry::Branch);

                        let quad1 = bounds.find_quadrant(&loc);
                        let quad2 = bounds.find_quadrant(&loc2);

                        // still collide
                        if quad1 == quad2 {
                            bounds = bounds.sub_bound(quad1);
                            child = match child.child(quad1) {
                                Ok(child) => child,
                                _ => panic!("overflow grouping not available yet")
                            }
                        }
                        // seperated after latest subdivision
                        else {
                            let child1 = child.child(quad1).unwrap();
                            let child2 = child.child(quad2).unwrap();

                            println!("New child: {}", child1);
                            println!("Old child: {} to {}", old_key, child2);
                            println!("-----------------------");

                            self.spatial_map.insert(child1, QuadtreeEntry::Leaf(loc));
                            self.spatial_map.insert(child2, QuadtreeEntry::Leaf(loc2));

                            *self.key_map.get_mut(invalid_key).unwrap() = child2;
                            return self.key_map.insert(child1);
                        }
                    }
                }
                // empty value, take coordinates for this spatial
                None => {
                    ret = child;
                    self.spatial_map.insert(ret, QuadtreeEntry::Leaf(loc));
                    println!("Inserted: {}", ret);
                    println!("-----------------------");
                    return self.key_map.insert(ret);
                }
            };
        };
    }

    pub fn remove(&mut self, key: SpatialKey) -> Option<S> {
        if let Some(k) = self.key_map.remove(key) {
            if let Some(QuadtreeEntry::Leaf(s)) = self.spatial_map.remove(&k) {

                // there are three possibilities:
                //  1: this location has two or more leaf siblings: do nothing as these points
                // need their current spatial levels to remain separated
                //  2: this location has at least one branch as a sibling: do nothing as
                // the deeper levels in that branch require being farther down in the tree
                //  3: this location has only one leaf sibling: reduce the level of that sibling
                // so that spatial complexity can be recovered from the removal of this element

                let top_level_quadrant = k.top_quadrant();
                let parent = k.parent().unwrap();
                let mut child_count = 0;
                let mut relocate = true;

                // check sibling quadrants for leaves or branches
                for (i, &quadrant) in QUADRANTS.iter().enumerate() {
                    if quadrant == top_level_quadrant {continue;}
                    let subparent = parent.child(quadrant).unwrap();
                    match self.spatial_map.get(&subparent) {
                        Some(QuadtreeEntry::Branch) => relocate = false,
                        Some(QuadtreeEntry::Leaf(_)) => child_count += 1,
                        _ => ()
                    }
                }

                if child_count != 1 || k.level() <= 1 {
                    relocate = false;
                }

                // if there is only one leaf sibling, we must relocate it up the tree.
                // otherwise do nothing
                if relocate {
                    let (mut key_to_move, mut s_to_move) = (None, None);
                    for &quadrant in QUADRANTS.iter() {
                        if quadrant == top_level_quadrant {continue;}
                        let key = parent.child(quadrant).unwrap();
                        if let Some(QuadtreeEntry::Leaf(s)) = self.spatial_map.remove(&key) {
                            key_to_move.replace(key);
                            s_to_move.replace(s);
                        }
                    };

                    let invalid_key = self.find_key_in_keymap(&key_to_move.unwrap()).unwrap();
                    let s_to_move = s_to_move.unwrap();
                    let mut parent = parent;

                    loop {
                        // safe to remove parent branch as we know there is nothing below it
                        self.spatial_map.remove(&parent);
                        let parents_parent = parent.parent();
                        if parents_parent.is_none() || parent.level() == 1 {
                            self.spatial_map.insert(parent, QuadtreeEntry::Leaf(s_to_move));

                            // validate key
                            *self.key_map.get_mut(invalid_key).unwrap() = parent;
                            break;
                        }

                        let parents_parent = parents_parent.unwrap();
                        if self.num_child(parents_parent) == 0 {
                            self.spatial_map.insert(parents_parent, QuadtreeEntry::Leaf(s_to_move));

                            // validate key
                            *self.key_map.get_mut(invalid_key).unwrap() = parent;
                            break;
                        }

                        parent = parents_parent;
                    }
                }
                return Some(s);
            }
        }
        None
    }

    pub fn neighbors(&self, key: Key) -> Vec<&S> {
        unimplemented!()
    }

    pub fn neighbors_mut(&mut self, key: Key) -> Vec<&mut S> {
        unimplemented!()
    }

    pub fn neighbors_within(&self, s: S, radius: f32) -> Vec<&S> { unimplemented!() }

    pub fn neighbors_within_mut(&mut self, s: S, radius: f32) -> Vec<&mut S> { unimplemented!() }

    pub fn values<'a>(&'a self) -> Vec<&'a S> {
        let mut ret = Vec::new();
        for key in self.key_map.keys() {
            if let QuadtreeEntry::Leaf(s) = self.spatial_map.get(self.key_map.get(key).unwrap()).unwrap(){
                ret.push(s);
            }
        }
        ret
    }

//    pub fn values_mut<'a>(&'a mut self) -> Vec<&'a mut S> {
//        let mut ret = Vec::new();
//        unsafe {
//            for key in self.key_map.keys() {
//                if let QuadtreeEntry::Leaf(s) = self.spatial_map.get_mut(self.key_map.get(key).unwrap()).unwrap() {
//                    ret.push(s);
//                }
//            }
//        }
//        ret
//    }

    /// Returns all bounds that make up the hierarchy of the quadtree
    pub fn bounds(&self) -> Vec<Bounds> {
        let mut ret = Vec::new();
        for key in self.spatial_map.keys() {
            ret.push(key.to_bounds(&self.space_boundary));
        }
        ret
    }

    /// Returns the bounds of every node on the tree that contains
    /// a Spatial element, skipping branches
    pub fn bounds_no_branch(&self) -> Vec<Bounds> {
        let mut ret = Vec::new();
        for (_, key) in &self.key_map {
            ret.push(key.to_bounds(&self.space_boundary));
        }
        ret
    }

    fn find_key_in_keymap(&mut self, spatial_key: &Key) -> Option<SpatialKey> {
        for (key, _spatial_key) in self.key_map.iter() {
            if(*_spatial_key == *spatial_key) {
                return Some(key);
            }
        }
        None
    }

    fn neighboring_keys(&self, key: Key) -> Vec<Key> {
        let mut ret = Vec::new();
        for same_size_key in key.compute_neighbors().iter(){
            if same_size_key.is_none() {continue;}
            let same_size_key = same_size_key.unwrap();
            match self.spatial_map.get(&same_size_key) {
                Some(QuadtreeEntry::Branch) => {

                }
                Some(QuadtreeEntry::Leaf(_)) => {
                    ret.push(key);
                }
                None => {
                    let mut parent_key = same_size_key.clone();
                    while !self.spatial_map.contains_key(&parent_key){
                        parent_key = parent_key.parent().unwrap();
                    }
                    ret.push(parent_key);
                }
            }
        }
        ret.sort_by(|a, b| a.cmp(b) );
        ret.dedup();
        ret
    }

    fn num_child(&self, key: Key) -> u32 {
        match self.spatial_map.get(&key) {
            Some(QuadtreeEntry::Leaf(_)) => 1,
            Some(QuadtreeEntry::Branch) => {
                let mut ret = 0;
                for quadrant in &crate::core::QUADRANTS {
                    if let Ok(child) = key.child(*quadrant) {
                        ret += self.num_child(child);
                    }
                }
                ret
            }
            _ => 0,
        }
    }
}