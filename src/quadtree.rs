use cgmath::{Aabb, Aabb2, Point2};
use std::clone::Clone;

pub const NODE_CAPACITY: usize = 16;

pub type Bounds = Aabb2<u32>;

pub trait HasCoordinates: Clone {
    fn coords(&self) -> Point2<u32>;
}

#[derive(Debug)]
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
                children: Box::new(None),
                level: 0
            },
            // Empty nodes don't count for size
            size: 0
        }
    }

    pub fn push(&mut self, value: T) {
        let success = self.root.insert(value);
        if !success { panic!("Could not insert value into tree!"); }
        self.size += 1;
    }
}

#[derive(Debug)]
struct QuadTreeNode<T> {
    values: Vec<T>,
    bounds: Bounds,
    children: Box<Option<[QuadTreeNode<T>; 4]>>,
    level: usize
}

impl<T: HasCoordinates> QuadTreeNode<T> {
    fn new(values: Vec<T>, bounds: Bounds, children: Option<[QuadTreeNode<T>; 4]>, level: usize) -> QuadTreeNode<T> {
        QuadTreeNode {
            values: values,
            bounds: bounds,
            children: Box::new(children),
            level: level
        }
    }

    fn subdivide(&mut self) {
        let x_size = self.bounds.dim().x / 2;
        let y_size = self.bounds.dim().y / 2;

        let x = self.bounds.min.x;
        let y = self.bounds.min.y;
        
        let ch = Some([
            QuadTreeNode::new(Vec::new(), Aabb2::new(Point2::new(x, y), Point2::new(x + x_size, y + y_size)), None, self.level + 1),
            QuadTreeNode::new(Vec::new(), Aabb2::new(Point2::new(x + x_size, y), Point2::new(x + x_size * 2, y + y_size)), None, self.level + 1),
            QuadTreeNode::new(Vec::new(), Aabb2::new(Point2::new(x, y + y_size), Point2::new(x + x_size, y + y_size * 2)), None, self.level + 1),
            QuadTreeNode::new(Vec::new(), Aabb2::new(Point2::new(x + x_size, y + y_size), Point2::new(x + x_size * 2, y + y_size * 2)), None, self.level + 1),
            ]);
        
        self.children = Box::new(ch);
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

        for c in self.children.as_mut().unwrap().iter_mut() {
            if c.insert(value.clone()) {
                return true;
            }
        }
        panic!("Insertion failed for unknown reason!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cgmath::{Point2, Aabb2};
    use person::{Status, Person};
    #[test]
    fn insert_values() {
        let mut tree = QuadTree::new(Aabb2::new(Point2::new(0, 0), Point2::new(100, 100)));
        let person = Person::new(0, Point2::new(12, 12), Status::Healthy);
        tree.push(person);
    }
}
