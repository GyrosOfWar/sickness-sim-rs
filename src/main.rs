extern crate cgmath;
extern crate rand;

use quadtree::*;
use person::{Person, Status};
use cgmath::{Aabb2, Point2};
use simulation::*;

pub mod person;
pub mod simulation;
pub mod quadtree;

#[allow(dead_code)]
mod constants {
    pub const POPULATION_SIZE: u32 = 30;    
    pub const INITIAL_INFECTED: u32 = 15;
    pub const INFLUENCE_RADIUS: f64 = 25.0;
    
    pub const INFECTIOUS_INFECTION_RATE: f64 = 0.01;
    pub const SICK_INFECTION_RATE: f64 = 0.01;
    pub const DEAD_INFECTION_RATE: f64 = 0.01;

    pub const ROOM_SIZE: u32 = 800;
    pub const MOVE_DISTANCE: u32 = 2;
    pub const CHANGE_DIRECTION_AFTER: u32 = 10;
    pub const TIME_INFECTIOUS: u32 = 50;
    pub const TIME_SICK: u32 = 20;
    pub const REMOVE_DEAD_AFTER: u32 = 20;
    pub const DIE_RATE: f64 = 0.0001;
}

#[derive(Debug, PartialEq, Clone)]
struct Value(Point2<u32>);

impl HasCoordinates for Value {
    fn coords(&self) -> Point2<u32> {
        match *self {
            Value(p) => p
        }
    }
}
    
fn main() {
    use rand::{Rng, thread_rng};
    use constants::*;
    
    let mut tree: QuadTree<Value> = QuadTree::new(Aabb2::new(Point2::new(0, 0), Point2::new(ROOM_SIZE, ROOM_SIZE)));

    let mut rand = thread_rng();

    for i in (0..100) {
        let x: f64 = rand.gen() * (ROOM_SIZE as f64);
        let y: f64 = rand.gen() * (ROOM_SIZE as f64);

        let v = Value(Point2::new(x as u32, y as u32));
        tree.push(v);
    }
    
    println!("{:?}", tree.root);
}
