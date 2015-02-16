extern crate cgmath;
extern crate rand;

use cgmath::Point2;
use person::{ObjectFactory, Status, Person};
use std::default::Default;

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

macro_rules! make_object {
    ($t: ty, [$($arg: expr), *]) => ($t::new($arg))
}

fn main() {
    let mut factory: ObjectFactory<Person> = ObjectFactory::new();

    for i in 0..10 {
        let mut person: Person = Default::default();
        factory.wrap(&mut person);
        println!("{}", person.id);
    }
}
