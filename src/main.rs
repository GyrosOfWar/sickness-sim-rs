#![feature(test)]

extern crate cgmath;
extern crate rand;
extern crate test;
extern crate ntree;

pub mod person;
pub mod simulation;
pub mod quadtree;

#[allow(dead_code)]
mod constants {
    pub const POPULATION_SIZE: u32 = 30;    
    pub const INITIAL_INFECTED: u32 = 15;
    pub const INFLUENCE_RADIUS: u32 = 25;
    
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

fn main() {
    //let mut ntree;
}
