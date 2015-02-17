use cgmath::{Aabb, Aabb2, Point2};
use std::clone::Clone;

pub const NODE_CAPACITY: usize = 16;

pub type Bounds = Aabb2<u32>;

pub trait HasCoordinates: Clone {
    fn coords(&self) -> Point2<u32>;
}

pub struct QuadTree<T> {
    root: QuadTreeNode<T>,
    size: usize
}

impl<T: HasCoordinates> QuadTree<T> {
    pub fn new(bounds: Bounds) -> QuadTree<T> {
        QuadTree {
            root: QuadTreeNode {
                values: Vec::with_capacity(NODE_CAPACITY),
                bounds: bounds,
                children: Box::new(None)
            },
            // Empty nodes don't count for size
            size: 0
        }
    }

    pub fn push(&mut self, value: T) {
        let success = self.root.insert(value);
        if !success { panic!("Could not insert value into tree!"); }
    }
}

struct QuadTreeNode<T> {
    values: Vec<T>,
    bounds: Bounds,
    children: Box<Option<[QuadTreeNode<T>; 4]>>
}

impl<T: HasCoordinates> QuadTreeNode<T> {
    fn new(values: Vec<T>, bounds: Bounds, children: Option<[QuadTreeNode<T>; 4]>) -> QuadTreeNode<T> {
        QuadTreeNode {
            values: values,
            bounds: bounds,
            children: Box::new(children)
        }
    }

    fn subdivide(&mut self) {
        // TODO
    }

    fn insert(&mut self, value: T) -> bool {
        if !self.bounds.contains(&value.coords()) {
            return false;
        }
        
        if self.values.len() < NODE_CAPACITY {
            self.values.push(value);
            return true;
        }

        if self.children.is_none() {
            self.subdivide();
        }

        // for c in self.children.unwrap().iter_mut() {
        //     if c.insert(value.clone()) {
        //         return true;
        //     }
        // }
        // assert!(false);
        false
    }
}
