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

    pub fn range_search(&self, range: Bounds) -> Vec<T> {
        self.root.range_search(range)
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
    a.min.x <= b.max.x &&
    b.min.x <= a.max.x &&
    a.min.y <= b.max.y &&
    b.min.y <= a.max.y
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
    use rand::{Rng, thread_rng};
    use constants::*;
    
    #[derive(Debug, PartialEq, Clone)]
    struct Value(Point2<u32>);
    
    impl HasCoordinates for Value {
        fn coords(&self) -> Point2<u32> {
            match *self {
                Value(p) => p
            }
        }
    }
    
    #[test]
    fn query_range() {
        let full_bound = Aabb2::new(Point2::new(0, 0), Point2::new(ROOM_SIZE, ROOM_SIZE));        
        let mut tree = QuadTree::new(full_bound);
        
        let mut rand = thread_rng();
        let mut points = Vec::new();
        // Add 100 random points
        for i in (0..100) {
            let x: f64 = rand.gen() * (ROOM_SIZE as f64);
            let y: f64 = rand.gen() * (ROOM_SIZE as f64);

            let v = Value(Point2::new(x as u32, y as u32));
            tree.push(v.clone());
            points.push(v);
        }
        let range_query = tree.range_search(full_bound);

        for p in range_query.iter() {
            assert!(points.contains(p));
        }
    }

    #[test]
    fn query_range_2() {
        
    }
}
