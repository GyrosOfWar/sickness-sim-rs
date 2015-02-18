use cgmath::Point2;
use constants::*;
use rand::{ThreadRng, Rng};
use std::num::Float;
use quadtree::HasCoordinates;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Status {
    Healthy,
    Infectious,
    Sick,
    Dead
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Direction {
    Top = 0,
    Right = 1,
    Down = 2,
    Left = 3
}

pub type Position = Point2<u32>;

#[derive(Debug, Clone)]
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
    pub fn new(id: usize, pos: Position, status: Status) -> Person {
        let t_infected = if status == Status::Healthy {
            None
        } else {
            Some(0)
        };

        Person {
            id: id,
            position: pos,
            status: status,
            facing_direction: Direction::Right,
            t_infected: t_infected,
            t_sick: None,
            t_died: None
        }
    }
    
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

    pub fn distance_to(&self, other: &Person) -> f64 {
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

impl HasCoordinates for Person {
    fn coords(&self) -> Point2<u32> {
        self.position
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;
    use cgmath::Point2;
    use constants::*;
    
    #[test]
    fn state_changes() {
        let mut rng = thread_rng();
        let mut person = Person::new(0, Point2::new(0, 0), Status::Infectious);
        for i in (1..TIME_INFECTIOUS+1) {
            person.tick(i, &mut rng);
            println!("t = {:?}, person.status: {:?}", i, person.status);
        }

        assert_eq!(person.status, Status::Sick)
    }
}
