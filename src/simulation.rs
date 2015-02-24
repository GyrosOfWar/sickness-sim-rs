use person::{Person, Status};
use constants::*;
use rand::{weak_rng, XorShiftRng, Rng};
use cgmath::Point2;
use ntree::NTree;
use quadtree::{QuadTreeRegion, QuadTree};

//#[derive(Debug)]
pub struct Simulation {
    population: QuadTree,
    rng: XorShiftRng,
    current_time: u32,
    full_range: QuadTreeRegion
}

impl Simulation {
    pub fn new() -> Simulation {
        let mut rng = weak_rng();
        let population = rng
            .gen_iter::<(f64, f64)>()
            .map(|(x, y)| Point2::new((x * ROOM_SIZE as f64) as u32, (y * ROOM_SIZE as f64) as u32))
            .enumerate()
            .map(|(i, pos)| {
                let status = if i < INITIAL_INFECTED as usize {
                    Status::Infectious
                } else {
                    Status::Healthy
                };
                Person::new(i, pos, status)
            })
            .take(POPULATION_SIZE as usize);

        let range = QuadTreeRegion::new(0, 0, ROOM_SIZE, ROOM_SIZE);
        let mut ntree = NTree::new(range.clone(), 16);

        for p in population {
            ntree.insert(p);
        }
        
        Simulation {
            population: ntree,
            rng: weak_rng(),
            current_time: 0,
            full_range: range
        }
            
    }

    pub fn tick(&mut self) -> bool {
        let mut full_pop: Vec<_> = self.population.range_query(&self.full_range).collect();
        // TODO remove dead people
        if self.is_finished(&full_pop) {
            return false;
        }

        let infection_r = INFLUENCE_RADIUS;
        for p in full_pop.iter_mut() {
            //p.tick(self.current_time, &mut self.rng);
            let bound = QuadTreeRegion::new(p.position.x - infection_r, p.position.y - infection_r,
                                            p.position.x + infection_r, p.position.y + infection_r);
            let infected = self.population
                .range_query(&bound)
                .filter(|q| q.distance_to(p) <= infection_r as f64);

            for i in infected {
                
            }
        }
        
        self.current_time += 1;
        true
    }

    fn is_finished(&self, population: &Vec<&Person>) -> bool {
        population.len() == 0 ||
            population
            .iter()
            .filter(|p| p.status != Status::Healthy)
            .count() == 0
   
    }
}

// TODO tests
