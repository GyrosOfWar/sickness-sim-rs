use cgmath::Point2;
use constants::*;
use rand::{ThreadRng, Rng};
use std::num::Float;

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Healthy,
    Infectious,
    Sick,
    Dead
}

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Top = 0,
    Right = 1,
    Down = 2,
    Left = 3
}

pub type Position = Point2<u32>;

#[derive(Debug)]
pub struct Person {
    pub id: usize,
    pub position: Position,
    pub status: Status,
    pub facing_direction: Direction,
    pub t_infected: Option<u32>,
    pub t_sick: Option<u32>,
    pub t_died: Option<u32>
}

impl Person {
    // TODO fn for is_infected(t: usize), is_sick(t: usize) etc.

    fn is_sick(&self, time: u32) -> bool {
        match self.t_infected {
            Some(inf) => time >= inf + TIME_INFECTIOUS,
            None => false
        }
    }

    fn is_dead(&self, time: u32, rng: &mut ThreadRng) -> bool {
        match self.t_sick {
            Some(sick) => {
                if time >= sick + TIME_SICK {
                    let t: f64 = rng.gen();
                    t <= DIE_RATE
                } else {
                    false
                }
            },
            None => false
        }
    }

    pub fn distance_to(&self, other: Person) -> f64 {
        let x1 = self.position.x as f64;
        let x2 = other.position.x as f64;
        let y1 = self.position.y as f64;
        let y2 = other.position.y as f64;

        ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt()
    }
    
    pub fn tick(&mut self, time: u32, rng: &mut ThreadRng) {
        // TODO
        match self.status {
            Status::Healthy => {},
            Status::Infectious => {
                if self.is_sick(time) {
                    self.status = Status::Sick;
                    self.t_sick = Some(time);
                }
            },
            Status::Sick => {
                if self.is_dead(time, rng) {
                    self.status = Status::Dead;
                    self.t_died = Some(time);
                }
            },
            Status::Dead => {}
        }
    }
}

pub struct PersonFactory {
    last_id: usize
}

impl PersonFactory {
    pub fn new() -> PersonFactory {
        PersonFactory {
            last_id: 0
        }
    }

    pub fn new_person(&mut self, status: Status, position: Position) -> Person {
        let person = Person {
            position: position,
            status: status,
            id: self.last_id,
            facing_direction: Direction::Right,
            t_infected: None,
            t_sick: None,
            t_died: None
        };

        self.last_id += 1;
        person
    }
}
 
