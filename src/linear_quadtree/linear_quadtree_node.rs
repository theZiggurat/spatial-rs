use slotmap::Key;
use hashbrown::HashMap;
use crate::quadrant::Quadrant;
use crate::Bounds;

pub(crate) mod consts {
    /// maximum depth of the tree using these nodes
    pub const RESOLUTION: u32 = 12;
    /// constant for neighbor calculation
    pub const T_X: u32 = 0b010101010101010101010101;
    /// constant for neighbor calculation
    pub const T_Y: u32 = 0b101010101010101010101010;
    /// constants for neighbor calculation
    pub const DIRECTION_INCREMENTS: [u32; 8] = [
        0b000000000000000000000001, // east
        0b000000000000000000000011, // north-east
        0b000000000000000000000010, // north
        0b010101010101010101010111, // north-west
        0b010101010101010101010101, // west
        0b111111111111111111111111, // south-west
        0b101010101010101010101010, // south
        0b101010101010101010101011  // south-east
    ];
}

/// Node used for indexing linear quadtrees
/// in constant time.
///
/// Based on the paper 'Finding Neighbors of Equal Size
/// in Linear Quadtrees and Octrees in Constant Time'
/// by Gunther Shrack (1991)
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LinearQuadTreeNode {
    /// [31:24]: level (8 bit unsigned)
    /// [23:0]: location (12 2-bit pairs)
    location: u32,
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

    /// Returns the location indexes of the 8 surrounding
    /// quadtree nodes of equal level.
    /// These are NOT gaurenteed to exist within
    /// the datastructure and must be checked for existence
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
                Node::new(mi, level)
            );
        }
        ret
    }

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

    pub fn coordinate(&self) -> u32 {
        self.location & 0xFFFFFF
    }

    pub fn level(&self) -> u32 {
        self.location >> 24
    }

}

#[cfg(test)]
mod test {
    use super::LinearQuadTreeNode;
    use crate::Quadrant;
    use slotmap::DefaultKey;

    #[test]
    fn test_location_level() {
        let node1 = LinearQuadTreeNode::from_location(
            0b00000111111001000000000000000000
        );
        let node2 = LinearQuadTreeNode::new(
            0b111001000000000000000000, 7
        );

        assert_eq!(node1, node2);

        assert_eq!(node.quadrant_at_level(1), Quadrant::TR);
        assert_eq!(node.quadrant_at_level(2), Quadrant::TL);
        assert_eq!(node.quadrant_at_level(3), Quadrant::BR);
        assert_eq!(node.quadrant_at_level(4), Quadrant::BL);
        assert_eq!(node.level(), 7);
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
}