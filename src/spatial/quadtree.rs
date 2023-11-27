use crate::shared::Rect;

pub trait Object2D {
    fn rect(&self) -> &Rect;
}

pub struct TreeNode<T: Object2D> {
    rect: Rect,
    children: Option<[Box<TreeNode<T>>; 4]>,
    objects: Vec<T>,
    depth: u32,
    max_depth: u32,
    max_objects: usize,
}

impl<T: Object2D> TreeNode<T> {
    pub fn new(rect: Rect, depth: u32, max_depth: u32, max_objects: usize) -> Self {
        Self {
            rect,
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
                if child.rect.contains(&object.rect()) {
                    child.insert(object);
                    return;
                }
            }

            self.objects.push(object);
        }
    }

    pub fn query(&self, rect: &Rect) -> Vec<&T> {
        let mut result = Vec::new();

        if self.rect.intersects(rect) {
            for object in self.objects.iter() {
                if rect.contains(&object.rect()) {
                    result.push(object);
                }
            }

            if let Some(children) = &self.children {
                for child in children.iter() {
                    result.append(&mut child.query(rect));
                }
            }
        }

        result
    }

    pub fn query_depth(&self, depth: u32) -> Vec<&T> {
        let mut result = Vec::new();

        if self.depth == depth {
            for object in self.objects.iter() {
                result.push(object);
            }
        } else if let Some(children) = &self.children {
            for child in children.iter() {
                result.append(&mut child.query_depth(depth));
            }
        }

        result
    }

    pub fn get_max_depth(&self, rect: &Rect) -> u32 {
        let mut result = 0;

        if self.rect.intersects(rect) {
            if let Some(children) = &self.children {
                for child in children.iter() {
                    result = result.max(child.get_max_depth(rect));
                }
            }
        }

        result
    }

    fn split(&mut self) {
        let half_width = self.rect.width / 2.0;
        let half_height = self.rect.height / 2.0;

        let top_left = Rect::new(self.rect.x, self.rect.y, half_width, half_height);

        let top_right = Rect::new(
            self.rect.x + half_width,
            self.rect.y,
            half_width,
            half_height,
        );

        let bottom_left = Rect::new(
            self.rect.x,
            self.rect.y + half_height,
            half_width,
            half_height,
        );

        let bottom_right = Rect::new(
            self.rect.x + half_width,
            self.rect.y + half_height,
            half_width,
            half_height,
        );

        self.children = Some([
            Box::new(TreeNode::new(
                top_left,
                self.depth + 1,
                self.max_depth,
                self.max_objects,
            )),
            Box::new(TreeNode::new(
                top_right,
                self.depth + 1,
                self.max_depth,
                self.max_objects,
            )),
            Box::new(TreeNode::new(
                bottom_left,
                self.depth + 1,
                self.max_depth,
                self.max_objects,
            )),
            Box::new(TreeNode::new(
                bottom_right,
                self.depth + 1,
                self.max_depth,
                self.max_objects,
            )),
        ]);

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

pub struct QuadTree<T: Object2D> {
    root: TreeNode<T>,
}

impl<T: Object2D> QuadTree<T> {
    pub fn new(rect: Rect, max_depth: u32, max_objects: usize) -> Self {
        Self {
            root: TreeNode::new(rect, 0, max_depth, max_objects),
        }
    }

    pub fn insert(&mut self, object: T) {
        self.root.insert(object);
    }

    pub fn query(&self, rect: &Rect) -> Vec<&T> {
        self.root.query(rect)
    }

    pub fn query_depth(&self, depth: u32) -> Vec<&T> {
        self.root.query_depth(depth)
    }

    pub fn get_max_depth(&self, rect: &Rect) -> u32 {
        self.root.get_max_depth(rect)
    }

    pub fn remove(&mut self, object: &T) {
        self.root.remove(object);
    }

    pub fn clear(&mut self) {
        self.root = TreeNode::new(
            self.root.rect,
            0,
            self.root.max_depth,
            self.root.max_objects,
        );
    }
}
