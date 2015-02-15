use cgmath::Point2;
use constants;

#[derive(Debug)]
pub enum Status {
    Healthy,
    Infectious,
    Sick,
    Dead
}

#[derive(Debug)]
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
    position: Position,
    status: Status,
    facing_direction: Direction
}

impl Person {
    pub fn tick(&mut self, time: u32) {
        
    }
}

pub struct PersonFactory {
    id_counter: usize
}

impl PersonFactory {
    pub fn new() -> PersonFactory {
        PersonFactory {
            id_counter: 0
        }
    }

    pub fn make_person(&mut self, pos: Position, status: Status) -> Person {
        let person = Person {
            id: self.id_counter,
            status: status,
            position: pos,
            facing_direction: Direction::Right,
        };

        self.id_counter += 1;
        person
    }
}
