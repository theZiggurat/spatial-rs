use crate::core::{Quadrant, Bounds};

mod consts {
    /// maximum depth of the tree using these nodes
    pub const RESOLUTION: u32 = 12;
    /// constant for neighbor calculation
    pub const T_X: u32 = 0x555555;
    /// constant for neighbor calculation
    pub const T_Y: u32 = 0xAAAAAA;
    /// constants for neighbor calculation
    pub const DIRECTION_INCREMENTS: [u32; 8] = [
        0x000001, // east
        0x000003, // north-east
        0x000002, // north
        0x555557, // north-west
        0x555555, // west
        0xFFFFFF, // south-west
        0xAAAAAA, // south
        0xAAAAAB  // south-east
    ];
}

/// Node used for indexing linear quadtrees
/// in constant time.
///
/// Based on the paper 'Finding Neighbors of Equal Size
/// in Linear Quadtrees and Octrees in Constant Time'
/// by Gunther Shrack (1991)
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct LinearQuadTreeNode {
    /// [31:28]: overflow | 4-bit unsigned
    ///                   | values: 0-16
    /// [27:24]: level    | 4 bit unsigned 
    ///                   | values: 1-12
    /// [23:0]: location  | 12 2-bit pairs 
    ///                   | values: 12
    location: u32,
}

impl Default for LinearQuadTreeNode {
    fn default() -> Self {
        Self { location: 0 }
    }
}

impl LinearQuadTreeNode {
    /// Creates new linear quad tree node based on location bits
    /// and level
    pub fn new(coordinate: u32, level: u32) -> LinearQuadTreeNode {
        let location = (level << 24) + coordinate;
        LinearQuadTreeNode::from_location(location)
    }

    /// Creates new linear quad tree from raw location data where
    /// location = coordinate + (level << 24), such that the location
    /// is the top 8 bits and the coordinate is the lower 24 bits
    pub fn from_location(location: u32) -> LinearQuadTreeNode {
        LinearQuadTreeNode {
            location
        }
    }

    /// Returns the keys of the 8 surrounding
    /// quadtree keys of equal level that may or may not exist.
    /// within the linearquadtree instance but can be verified in
    /// 0(1) time
    pub fn compute_neighbors(&self) -> [Option<LinearQuadTreeNode>; 8] {
        let mut ret = [None; 8];

        let location = self.coordinate();
        let level = self.level();

        for i in 0..8 {
            let ni = location;
            let delta_ni = consts::DIRECTION_INCREMENTS[i] <<
                (2 * (consts::RESOLUTION - level));

            let mi =
                (((ni | consts::T_Y) + (delta_ni & consts::T_X)) & consts::T_X) |
                (((ni | consts::T_X) + (delta_ni & consts::T_Y)) & consts::T_Y);

            ret[i].replace(
                LinearQuadTreeNode::new(mi, level)
            );
        }
        ret
    }

    #[inline(always)]
    pub fn quadrant_at_level(&self, level: u32) -> Quadrant {
        assert!(level < consts::RESOLUTION);

        let location_masked = (self.location >> ((12 - level) * 2)) & 0b11;
        match location_masked {
            0b00 => Quadrant::BL,
            0b01 => Quadrant::BR,
            0b10 => Quadrant::TL,
            0b11 => Quadrant::TR,
            _ => unreachable!()
        }
    }

    pub fn coordinate_in_quadrants(&self) -> Vec<Quadrant> {
        let mut ret = vec![];
        for i in 0..self.level() {
            ret.push(self.quadrant_at_level(i+1));
        }
        ret
    }

    #[inline(always)]
    pub fn coordinate(&self) -> u32 {
        self.location & 0xFFFFFF
    }

    #[inline(always)]
    pub fn level(&self) -> u32 {
        self.location >> 24 & 0xF
    }


    /// Returns some overflow identifier for unique 
    /// identification of keys that belong to the same
    /// location
    /// Or none if there is no overflow
    #[inline(always)]
    pub fn overflow(&self) -> Option<u32> {
        let ret = self.location >> 28;
        match ret {
            1..=15 => Some(ret),
            0 => None,
            _ => unreachable!()
        }
    }

    pub fn increment_overflow(&mut self) {
        let overflow = self.overflow().unwrap_or(0) + 1;
        assert_ne!(overflow, 16);
        let mask = !(0xF << 28);
        self.location = (self.location & mask) | overflow;
    }

    pub fn unit_bounds(&self) -> Bounds {
        let mut bounds = Bounds::new(0., 0., 1., 1.);
        for quadrant in self.coordinate_in_quadrants() {
            bounds = bounds.sub_bound(quadrant);
        }
        bounds
    }

    /// mutates this key to represent further subdivision
    /// of location based on input quadrant
    /// (subset of self)
    pub fn write_level(&mut self, quadrant: Quadrant){
        let bits = match quadrant {
            Quadrant::BL => 0b00,
            Quadrant::BR => 0b01,
            Quadrant::TL => 0b10,
            Quadrant::TR => 0b11
        } >> (self.level() * 2);

        let mask = !(0b11 << (self.level() * 2));
        self.location = (self.location & mask) | bits;

        let bits = (self.level() + 1) << 24;
        assert!(bits <= 12);

        let mask = !(0xF << 24);
        self.location = (self.location & mask) | bits;
    }

    /// mutates this key to represent location one level down
    /// (superset of self)
    pub fn remove_level(&mut self) {
        let mask = !(0b11 << (self.level() * 2));
        self.location = self.location & mask;

        let bits = self.level().checked_sub(1).unwrap_or(0) << 24;
        let mask = !(0xF << 24);
        self.location = (self.location & mask) | bits;
    }

    /// returns new key that is one level deeper than self within
    /// certain input quadrant. If resolution limit is reached, overflow
    /// will be incremented
    pub fn child(&self, quadrant: Quadrant) -> Self {
        let ret = self.clone();
        if ret.overflow().is_some() || ret.level() == consts::RESOLUTION {
            ret.increment_overflow();
        } else {
            ret.write_level(quadrant);
        }
        ret
    }

    /// Returns new key that is one level above self
    pub fn parent(&self) -> Self {
        let ret = self.clone();
        ret.remove_level();
        ret
    }

}

#[cfg(test)]
mod test {
    use super::LinearQuadTreeNode;
    use crate::core::Quadrant;

    #[test]
    fn test_location_level() {
        let node1 = LinearQuadTreeNode::from_location(
            0b00000111111001000000000000000000
        );
        let node2 = LinearQuadTreeNode::new(
            0b111001000000000000000000, 7
        );

        assert_eq!(node1, node2);

        assert_eq!(node1.quadrant_at_level(1), Quadrant::TR);
        assert_eq!(node1.quadrant_at_level(2), Quadrant::TL);
        assert_eq!(node1.quadrant_at_level(3), Quadrant::BR);
        assert_eq!(node1.quadrant_at_level(4), Quadrant::BL);
        assert_eq!(node1.level(), 7);
    }

    #[test]
    fn test_quadtree_neighbors() {
        // initialize node in:
        // level 1: 01 (Bottom right)
        // level 2: 10 (Top left)
        let node = LinearQuadTreeNode::new(
            0b011000000000000000000000, 2
        );

        // verify the comment above
        assert_eq!(node.coordinate_in_quadrants(), vec![Quadrant::BR, Quadrant::TL]);

        // verify that neighbors are correct
        // note these neighbors are guaranteed to be of level 2
        // thus the corresponding vec will be 2 long
        let neighbors = node.compute_neighbors();
        assert_eq!(neighbors[0].unwrap().coordinate_in_quadrants(),
                   vec![Quadrant::BR, Quadrant::TR], "east");
        assert_eq!(neighbors[1].unwrap().coordinate_in_quadrants(),
                   vec![Quadrant::TR, Quadrant::BR], "north-east");
        assert_eq!(neighbors[2].unwrap().coordinate_in_quadrants(),
                   vec![Quadrant::TR, Quadrant::BL], "north");
        assert_eq!(neighbors[3].unwrap().coordinate_in_quadrants(),
                   vec![Quadrant::TL, Quadrant::BR], "north-west");
        assert_eq!(neighbors[4].unwrap().coordinate_in_quadrants(),
                   vec![Quadrant::BL, Quadrant::TR], "west");
        assert_eq!(neighbors[5].unwrap().coordinate_in_quadrants(),
                   vec![Quadrant::BL, Quadrant::BR], "south-west");
        assert_eq!(neighbors[6].unwrap().coordinate_in_quadrants(),
                   vec![Quadrant::BR, Quadrant::BL], "south");
        assert_eq!(neighbors[7].unwrap().coordinate_in_quadrants(),
                   vec![Quadrant::BR, Quadrant::BR], "south-east");

    }

    #[test]
    fn test_write_level() {
        let mut node: LinearQuadTreeNode = Default::default();
        node.write_level(Quadrant::BL);
        assert_eq!(node.level(), 1);
        node.write_level(Quadrant::BR);
        assert_eq!(node.level(), 2);
        assert_eq!(node.coordinate_in_quadrants(), 
            vec![Quadrant::BL, Quadrant::BR]);

    }
}