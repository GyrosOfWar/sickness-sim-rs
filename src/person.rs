use cgmath::Point2;
use constants;
use std::default::Default;

#[derive(Debug)]
pub enum Status {
    Healthy,
    Infectious,
    Sick,
    Dead
}

impl Default for Status {
    fn default() -> Status {
        Status::Healthy
    }
}

#[derive(Debug)]
pub enum Direction {
    Top = 0,
    Right = 1,
    Down = 2,
    Left = 3
}

impl Default for Direction {
    fn default() -> Direction {
        Direction::Right
    }
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

impl Default for Person {
    fn default() -> Person {
        Person {
            id: 0,
            position: Point2::new(0, 0),
            status: Status::Healthy,
            facing_direction: Direction::Right
        }
    }
}

impl HasId for Person {
    fn id(&self) -> usize {
        self.id
    }
    
    fn set_id(&mut self, value: usize) {
        self.id = value;
    }
}

pub trait HasId {
    fn id(&self) -> usize;
    fn set_id(&mut self, value: usize);
}

pub struct ObjectFactory<T> {
    last_id: usize
}

impl<T: HasId> ObjectFactory<T> {
    pub fn new() -> ObjectFactory<T> {
        ObjectFactory {
            last_id: 0
        }
    }

    pub fn wrap(&mut self, thing: &mut T) {
        thing.set_id(self.last_id);
        self.last_id += 1;
    }
}
