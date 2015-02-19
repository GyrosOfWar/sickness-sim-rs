use cgmath::{Aabb, Aabb2, Point2, Point};
use std::clone::Clone;

pub const NODE_CAPACITY: usize = 16;

pub type Bounds = Aabb2<u32>;

pub trait HasCoordinates: Clone {
    fn coords(&self) -> Point2<u32>;
}

#[derive(Debug)]
pub struct QuadTree<T> {
    // TODO make private after debugging
    pub root: QuadTreeNode<T>,
    size: usize
}

impl<T: HasCoordinates> QuadTree<T> {
    pub fn new(bounds: Bounds) -> QuadTree<T> {
        QuadTree {
            root: QuadTreeNode {
                values: Vec::with_capacity(NODE_CAPACITY),
                bounds: bounds,
                nw: Box::new(None),
                ne: Box::new(None),
                sw: Box::new(None),
                se: Box::new(None),
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

// TODO make private after debugging
#[derive(Debug)]
pub struct QuadTreeNode<T> {
    pub values: Vec<T>,
    bounds: Bounds,
    nw: Box<Option<QuadTreeNode<T>>>,
    ne: Box<Option<QuadTreeNode<T>>>,
    se: Box<Option<QuadTreeNode<T>>>,
    sw: Box<Option<QuadTreeNode<T>>>,
    level: usize
}

/* 
a.x <= b.x + b.width &&
b.x <= a.x + a.width &&
a.y <= b.y + b.height &&
b.y <= a.y + a.height;
*/

fn bounds_intersecting(a: Bounds, b: Bounds) -> bool {
    let ax = a.min.x;
    let ay = a.min.y;
    let bx = b.min.x;
    let by = b.min.y;

    let a_xs = a.dim().x;
    let a_ys = a.dim().y;

    let b_xs = b.dim().x;
    let b_ys = b.dim().y;

    ax <= bx + b_xs &&
        bx <= ax + a_xs &&
        ay <= by + b_ys &&
        by <= ay + a_ys
}

impl<T: HasCoordinates> QuadTreeNode<T> {
    fn new(values: Vec<T>, bounds: Bounds, level: usize) -> QuadTreeNode<T> {
        QuadTreeNode {
            values: values,
            bounds: bounds,
            nw: Box::new(None),
            ne: Box::new(None),
            sw: Box::new(None),
            se: Box::new(None),
            level: level
        }
    }

    fn empty(bounds: Bounds, level: usize) -> QuadTreeNode<T> {
        QuadTreeNode {
            values: Vec::with_capacity(NODE_CAPACITY),
            bounds: bounds,
            nw: Box::new(None),
            ne: Box::new(None),
            sw: Box::new(None),
            se: Box::new(None),
            level: level
        }
    }

    fn subdivide(&mut self) {
        let xs = self.bounds.dim().x;
        let ys = self.bounds.dim().y;

        let xs_half = xs / 2;
        let ys_half = ys / 2;
        
        let x = self.bounds.min.x;
        let y = self.bounds.min.y;

        let nw_box = Aabb::new(Point2::new(x, y), Point2::new(x + xs_half, y + ys_half));
        let ne_box = Aabb::new(Point2::new(x + xs_half, y), Point2::new(x + xs, y + ys_half));
        let sw_box = Aabb::new(Point2::new(x, y + ys_half), Point2::new(x + xs_half, y + ys));
        let se_box = Aabb::new(Point2::new(x + xs_half, y + ys_half), Point2::new(x + xs, y + ys));

        self.nw = Box::new(Some(QuadTreeNode::empty(nw_box, self.level + 1)));
        self.ne = Box::new(Some(QuadTreeNode::empty(ne_box, self.level + 1)));
        self.sw = Box::new(Some(QuadTreeNode::empty(sw_box, self.level + 1)));
        self.se = Box::new(Some(QuadTreeNode::empty(se_box, self.level + 1)));
    }

    fn insert(&mut self, value: T) -> bool {
        println!("Inserting value at level {}!", self.level);
        
        if !self.bounds.contains(&value.coords()) {
            println!("Value does not fit!");
            return false;
        }
        
        if self.values.len() < NODE_CAPACITY {
            self.values.push(value);
            return true;
        }

        if !self.nw.is_some() {
            self.subdivide();
        }

        if self.nw.as_mut().unwrap().insert(value.clone()) {
            return true;
        }
        if self.ne.as_mut().unwrap().insert(value.clone()) {
            return true;
        }
        if self.sw.as_mut().unwrap().insert(value.clone()) {
            return true;
        }
        if self.ne.as_mut().unwrap().insert(value.clone()) {
            return true;
        }
        
        panic!("Insertion failed for unknown reason!")
    }

    fn range_search(&self, bounds: Bounds) -> Vec<T> {
        let mut result = Vec::new();

        if !bounds_intersecting(self.bounds, bounds) {
            return result;
        }

        for p in self.values.iter() {
            if bounds.contains(&p.coords()) {
                result.push(p.clone());
            }
        }

        if self.nw.is_none() {
            return result;
        }
        result.push_all(&self.nw.as_ref().unwrap().range_search(bounds)[]);
        result.push_all(&self.ne.as_ref().unwrap().range_search(bounds)[]);
        result.push_all(&self.sw.as_ref().unwrap().range_search(bounds)[]);
        result.push_all(&self.ne.as_ref().unwrap().range_search(bounds)[]);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cgmath::{Point2, Aabb2};
    use person::{Status, Person};
    
    #[test]
    fn insert_values() {
        let mut tree: QuadTree<Person> = QuadTree::new(Aabb2::new(Point2::new(0, 0), Point2::new(100, 100)));
        let person = Person::new(0, Point2::new(12, 12), Status::Healthy);
        tree.push(person);
    }
}
