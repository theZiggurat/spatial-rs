struct Octree<T: Spatial3D> {
    containter: Vec<T>,

}

impl Octree<T: Spatial3D> {
    fn new() {
        Octree {
            container: vec![],

        }
    }
}