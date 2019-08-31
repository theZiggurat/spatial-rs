
mod quad;
mod spatial;
pub use crate::quad::{Quadtree, Bounds, QuadtreeNode, Branch, Quadrant};
pub use crate::spatial::{Spatial2D, Spatial3D, Point2D};

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_quadtree_insert() {

        let bounds = Bounds::new(-1., 1., -1., 1.);
        let mut quadtree = Quadtree::<Point2D>::new(bounds);
        quadtree.insert(Point2D::new(-0.5,0.5));
        quadtree.insert(Point2D::new(0.5,-0.5));
        assert_eq!(quadtree.container, vec![Point2D::new(-0.5,0.5), Point2D::new(0.5,-0.5)]);
    }

    #[test]
    fn test_quadtree_branches() {

        let bounds = Bounds::new(-1., 1., -1., 1.);
        let mut quadtree = Quadtree::<Point2D>::new(bounds);
        quadtree.insert(Point2D::new(-0.5,0.5));
        quadtree.insert(Point2D::new(0.5,-0.5));

        let quadtree_root = {

            let mut qtr = QuadtreeNode::<Point2D>::new_branch();
            qtr.insert_in_branch(0usize,
                                 Quadrant::BL,
                                 bounds.sub_bound(Quadrant::TL),
                                 &quadtree.container);

            qtr.insert_in_branch(1usize,
                                 Quadrant::TR,
                                 bounds.sub_bound(Quadrant::TL),
                                 &quadtree.container);
            qtr
        };
        assert_eq!(quadtree.root, quadtree_root);
    }
}