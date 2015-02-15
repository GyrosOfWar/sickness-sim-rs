extern crate cgmath;
extern crate rand;

use cgmath::Point2;
use person::{PersonFactory, Status};

pub mod person;
pub mod simulation;

#[allow(dead_code)]
mod constants {
    pub const POPULATION_SIZE: u32 = 2000;    
    pub const INITIAL_INFECTED: u32 = 15;
    pub const INFLUENCE_RADIUS: u32 = 25;
    pub const INFECTIOUS_INFECTION_RATE: f64 = 0.01;
    pub const SICK_INFECTION_RATE: f64 = 0.01;
    pub const DEAD_INFECTION_RATE: f64 = 0.01;
    pub const ROOM_SIZE: u32 = 800;
    pub const MOVE_DISTANCE: u32 = 2;
    pub const CHANGE_DIRECTION_AFTER: u32 = 10;
    pub const TIME_INFECTIOUS: u32 = 50;
}

fn main() {
    let mut factory = PersonFactory::new();

    for i in 0..10 {
        let p = factory.make_person(Point2::new(2, 2), Status::Healthy);
        println!("p.id = {}", p.id);
    }
}
