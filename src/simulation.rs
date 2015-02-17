use person::{Person, Status};
use constants::*;
use rand::{thread_rng, ThreadRng, Rng};
use cgmath::Point2;

//#[derive(Debug)]
pub struct Simulation {
    pub population: Vec<Person>,
    rng: ThreadRng,
    current_time: u32
}

impl Simulation {
    pub fn new() -> Simulation {
        let mut rng = thread_rng();
        let population: Vec<Person> = rng
            .gen_iter::<(u32, u32)>()
            .map(|(x, y)| Point2::new(x * ROOM_SIZE, y * ROOM_SIZE ))
            .enumerate()
            .map(|(i, pos)| {
                let status = if i < INITIAL_INFECTED as usize {
                    Status::Infectious
                } else {
                    Status::Healthy
                };
                Person::new(i, pos, status)
            })
            .take(POPULATION_SIZE as usize)
            .collect();
            
        Simulation {
            population: population,
            rng: rng,
            current_time: 0
        }
    }

    pub fn tick(&mut self) {
        for p in self.population.iter_mut() {
            // let neighbors: Vec<_> = self.population
            //     .iter()
            //     .filter(|q| q.status == Status::Healthy && p.distance_to(q) <= INFLUENCE_RADIUS)
            //     .map(|q| q.clone())
            //     .collect();
            // for q in neighbors {
            //     println!("{:?}", q);
            // }
            p.tick(self.current_time, &mut self.rng);
            
        }

        self.current_time += 1;
    }

    pub fn is_finished(&self) -> bool {
        self.population.len() == 0 ||
            self.population
            .iter()
            .filter(|p| p.status != Status::Healthy)
            .count() == 0
    }
}

#[test]
fn create_population() {
    let mut simulation = Simulation::new();
    let infected_count = simulation.population.iter().filter(|p| p.status == Status::Infectious).count();
    assert_eq!(infected_count, INITIAL_INFECTED as usize);
}
