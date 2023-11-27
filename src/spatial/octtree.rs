use crate::shared::Bounds;

pub trait Object3D {
    fn bounds(&self) -> &Bounds;
}

pub struct TreeNode<T: Object3D> {
    bounds: Bounds,
    children: Option<[Box<TreeNode<T>>; 8]>,
    objects: Vec<T>,
    depth: u32,
    max_depth: u32,
    max_objects: usize,
}

impl<T: Object3D> TreeNode<T> {
    pub fn new(bounds: Bounds, depth: u32, max_depth: u32, max_objects: usize) -> Self {
        Self {
            bounds,
            children: None,
            objects: Vec::new(),
            depth,
            max_depth,
            max_objects,
        }
    }

    pub fn insert(&mut self, object: T) {
        if self.depth == self.max_depth || self.objects.len() < self.max_objects {
            self.objects.push(object);
        } else {
            if self.children.is_none() {
                self.split();
            }

            for child in self.children.as_mut().unwrap().iter_mut() {
                if child.bounds.contains(&object.bounds()) {
                    child.insert(object);
                    return;
                }
            }

            self.objects.push(object);
        }
    }

    pub fn query(&self, bounds: &Bounds) -> Vec<&T> {
        let mut result = Vec::new();

        if self.bounds.intersects(bounds) {
            for object in self.objects.iter() {
                if bounds.contains(&object.bounds()) {
                    result.push(object);
                }
            }

            if let Some(children) = &self.children {
                for child in children.iter() {
                    result.append(&mut child.query(bounds));
                }
            }
        }

        result
    }

    fn split(&mut self) {
        let half_size = self.bounds.size() / 2.0;
        let children = [
            Box::new(TreeNode::new(
                Bounds::new(self.bounds.min, self.bounds.min + half_size),
                self.depth + 1,
                self.max_depth,
                self.max_objects,
            )),
            Box::new(TreeNode::new(
                Bounds::new(
                    self.bounds.min + glam::Vec3::new(half_size.x, 0.0, 0.0),
                    self.bounds.min + glam::Vec3::new(half_size.x, half_size.y, 0.0),
                ),
                self.depth + 1,
                self.max_depth,
                self.max_objects,
            )),
            Box::new(TreeNode::new(
                Bounds::new(
                    self.bounds.min + glam::Vec3::new(0.0, half_size.y, 0.0),
                    self.bounds.min + glam::Vec3::new(half_size.x, half_size.y, half_size.z),
                ),
                self.depth + 1,
                self.max_depth,
                self.max_objects,
            )),
            Box::new(TreeNode::new(
                Bounds::new(
                    self.bounds.min + glam::Vec3::new(half_size.x, half_size.y, 0.0),
                    self.bounds.min + glam::Vec3::new(half_size.x, half_size.y, half_size.z),
                ),
                self.depth + 1,
                self.max_depth,
                self.max_objects,
            )),
            Box::new(TreeNode::new(
                Bounds::new(
                    self.bounds.min + glam::Vec3::new(0.0, 0.0, half_size.z),
                    self.bounds.min + glam::Vec3::new(half_size.x, half_size.y, half_size.z),
                ),
                self.depth + 1,
                self.max_depth,
                self.max_objects,
            )),
            Box::new(TreeNode::new(
                Bounds::new(
                    self.bounds.min + glam::Vec3::new(half_size.x, 0.0, half_size.z),
                    self.bounds.min + glam::Vec3::new(half_size.x, half_size.y, half_size.z),
                ),
                self.depth + 1,
                self.max_depth,
                self.max_objects,
            )),
            Box::new(TreeNode::new(
                Bounds::new(
                    self.bounds.min + glam::Vec3::new(0.0, half_size.y, half_size.z),
                    self.bounds.min + glam::Vec3::new(half_size.x, half_size.y, half_size.z),
                ),
                self.depth + 1,
                self.max_depth,
                self.max_objects,
            )),
            Box::new(TreeNode::new(
                Bounds::new(self.bounds.min + half_size, self.bounds.max),
                self.depth + 1,
                self.max_depth,
                self.max_objects,
            )),
        ];

        self.children = Some(children);

        let mut objects = Vec::new();
        std::mem::swap(&mut objects, &mut self.objects);

        for object in objects {
            self.insert(object);
        }
    }

    pub fn remove(&mut self, object: &T) {
        if let Some(children) = &mut self.children {
            for child in children.iter_mut() {
                child.remove(object);
            }
        } else {
            self.objects.retain(|o| o as *const T != object as *const T);
        }
    }
}

pub struct OctTree<T: Object3D> {
    root: TreeNode<T>,
}

impl<T: Object3D> OctTree<T> {
    pub fn new(bounds: Bounds, max_depth: u32, max_objects: usize) -> Self {
        Self {
            root: TreeNode::new(bounds, 0, max_depth, max_objects),
        }
    }

    pub fn insert(&mut self, object: T) {
        self.root.insert(object);
    }

    pub fn query(&self, bounds: &Bounds) -> Vec<&T> {
        self.root.query(bounds)
    }

    pub fn remove(&mut self, object: &T) {
        self.root.remove(object);
    }

    pub fn clear(&mut self) {
        self.root = TreeNode::new(
            self.root.bounds,
            0,
            self.root.max_depth,
            self.root.max_objects,
        );
    }
}
