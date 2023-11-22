use super::{frustum::Frustum, SpriteId};
use crate::{
    ecs::EntityId,
    shared::{Bounds, Rect, ResourceType},
};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

const MAX_DEPTH: u32 = 5;
const MAX_OBJECTS: usize = 20;
const ROOT_RECT: Rect = Rect::new(-1000.0, -1000.0, 2000.0, 2000.0);

pub struct SceneTree2d {
    trees: HashMap<ResourceType, Box<dyn Any>>,
}

impl SceneTree2d {
    pub fn new() -> Self {
        Self {
            trees: HashMap::new(),
        }
    }

    pub fn insert<T: SpatialObject2d + 'static>(&mut self, object: T) {
        let resource_type = TypeId::of::<T>().into();
        let tree = self
            .trees
            .entry(resource_type)
            .or_insert_with(|| Box::new(QuadTree::<T>::new(ROOT_RECT, MAX_DEPTH, MAX_OBJECTS)));
        tree.downcast_mut::<QuadTree<T>>().unwrap().insert(object);
    }

    pub fn remove<T: SpatialObject2d + 'static>(&mut self, object: &T) {
        let resource_type = TypeId::of::<T>().into();
        if let Some(tree) = self.trees.get_mut(&resource_type) {
            tree.downcast_mut::<QuadTree<T>>().unwrap().remove(object);
        }
    }

    pub fn query<T: SpatialObject2d + 'static>(&self, rect: &Rect) -> Vec<&T> {
        let resource_type = TypeId::of::<T>().into();
        if let Some(tree) = self.trees.get(&resource_type) {
            tree.downcast_ref::<QuadTree<T>>().unwrap().query(rect)
        } else {
            Vec::new()
        }
    }
}

pub trait Draw2d: SpatialObject2d + 'static {}

pub struct DrawSprite {
    pub sprite: SpriteId,
    pub transform: glam::Mat4,
    pub rect: Rect,
}

impl DrawSprite {
    pub fn new(sprite: SpriteId, transform: glam::Mat4, rect: Rect) -> Self {
        Self {
            sprite,
            transform,
            rect,
        }
    }
}

impl SpatialObject2d for DrawSprite {
    fn bounds(&self) -> Rect {
        self.rect
    }
}

impl Draw2d for DrawSprite {}

pub trait SpatialObject2d {
    fn bounds(&self) -> Rect;
}

pub struct QuadNode<T: SpatialObject2d> {
    pub children: Option<[Box<QuadNode<T>>; 4]>,
    pub objects: Vec<T>,
    pub rect: Rect,
    pub depth: u32,
    pub max_depth: u32,
    pub max_objects: usize,
}

impl<T: SpatialObject2d> QuadNode<T> {
    pub fn new(rect: Rect, depth: u32, max_depth: u32, max_objects: usize) -> Self {
        Self {
            children: None,
            objects: Vec::new(),
            rect,
            depth,
            max_depth,
            max_objects,
        }
    }

    pub fn insert(&mut self, object: T) {
        if self.depth == self.max_depth || self.objects.len() < self.max_objects {
            self.objects.push(object);
        } else {
            let rect = object.bounds();
            let center = rect.center();
            let mut index = 0;
            if center.x > self.rect.center().x {
                index += 1;
            }
            if center.y > self.rect.center().y {
                index += 2;
            }
            if self.children.is_none() {
                self.children = Some([
                    Box::new(QuadNode::new(
                        Rect::new(
                            self.rect.x,
                            self.rect.y,
                            self.rect.width / 2.0,
                            self.rect.height / 2.0,
                        ),
                        self.depth + 1,
                        self.max_depth,
                        self.max_objects,
                    )),
                    Box::new(QuadNode::new(
                        Rect::new(
                            self.rect.x + self.rect.width / 2.0,
                            self.rect.y,
                            self.rect.width / 2.0,
                            self.rect.height / 2.0,
                        ),
                        self.depth + 1,
                        self.max_depth,
                        self.max_objects,
                    )),
                    Box::new(QuadNode::new(
                        Rect::new(
                            self.rect.x,
                            self.rect.y + self.rect.height / 2.0,
                            self.rect.width / 2.0,
                            self.rect.height / 2.0,
                        ),
                        self.depth + 1,
                        self.max_depth,
                        self.max_objects,
                    )),
                    Box::new(QuadNode::new(
                        Rect::new(
                            self.rect.x + self.rect.width / 2.0,
                            self.rect.y + self.rect.height / 2.0,
                            self.rect.width / 2.0,
                            self.rect.height / 2.0,
                        ),
                        self.depth + 1,
                        self.max_depth,
                        self.max_objects,
                    )),
                ]);
            }
            self.children.as_mut().unwrap()[index].insert(object);
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

    pub fn query(&self, rect: &Rect) -> Vec<&T> {
        let mut result = Vec::new();
        if self.rect.intersects(rect) {
            for object in &self.objects {
                if object.bounds().intersects(rect) {
                    result.push(object);
                }
            }
            if let Some(children) = &self.children {
                for child in children {
                    result.append(&mut child.query(rect));
                }
            }
        }
        result
    }

    pub fn query_depth(&self, depth: u32) -> Vec<&T> {
        let mut result = Vec::new();
        if self.depth == depth {
            for object in &self.objects {
                result.push(object);
            }
        } else if let Some(children) = &self.children {
            for child in children {
                result.append(&mut child.query_depth(depth));
            }
        }
        result
    }

    pub fn get_max_depth(&self, rect: &Rect) -> u32 {
        if self.rect.intersects(rect) {
            if let Some(children) = &self.children {
                let mut result = 0;
                for child in children {
                    result = result.max(child.get_max_depth(rect));
                }
                result
            } else {
                self.depth
            }
        } else {
            0
        }
    }
}

pub struct QuadTree<T: SpatialObject2d> {
    root: QuadNode<T>,
}

impl<T: SpatialObject2d> QuadTree<T> {
    pub fn new(rect: Rect, max_depth: u32, max_objects: usize) -> Self {
        Self {
            root: QuadNode::new(rect, 0, max_depth, max_objects),
        }
    }

    pub fn insert(&mut self, object: T) {
        self.root.insert(object);
    }

    pub fn remove(&mut self, object: &T) {
        self.root.remove(object);
    }

    pub fn query(&self, rect: &Rect) -> Vec<&T> {
        self.root.query(rect)
    }

    pub fn get_max_depth(&self, rect: &Rect) -> u32 {
        self.root.get_max_depth(rect)
    }

    pub fn query_depth(&self, depth: u32) -> Vec<&T> {
        self.root.query_depth(depth)
    }
}
