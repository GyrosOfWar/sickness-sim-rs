use ntree::{NTree, Region};
use person::Person;
use cgmath::Point2;

pub type QuadTree = NTree<QuadTreeRegion, Person>;

#[derive(Clone, Debug, PartialEq)]
pub struct QuadTreeRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32
}

impl QuadTreeRegion {
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> QuadTreeRegion {
        QuadTreeRegion {
            x: x, y: y, height: h, width: w
        }
    }
}

impl Region<Person> for QuadTreeRegion {
    fn contains(&self, p: &Person) -> bool {
        self.x <= p.position.x && self.y <= p.position.y
            && (self.x + self.width) >= p.position.x && (self.y + self.height) >= p.position.y
    }

    fn split(&self) -> Vec<QuadTreeRegion> {
        let halfwidth = self.width / 2;
        let halfheight = self.height / 2;
        vec![
            QuadTreeRegion {
                x: self.x,
                y: self.y,
                width: halfwidth,
                height: halfheight
            },
            
            QuadTreeRegion {
                x: self.x,
                y: self.y + halfheight,
                width: halfwidth,
                height: halfheight
            },
            
            QuadTreeRegion {
                x: self.x + halfwidth,
                y: self.y,
                width: halfwidth,
                height: halfheight
            },
            
            QuadTreeRegion {
                x: self.x + halfwidth,
                y: self.y + halfheight,
                width: halfwidth,
                height: halfheight
            }
        ]
    }

    fn overlaps(&self, other: &QuadTreeRegion) -> bool {
        self.x <= other.x + other.width &&
            other.x <= self.x + self.width &&
            self.y <= other.y + other.height &&
            other.y <= self.y + self.height            
    }
}
