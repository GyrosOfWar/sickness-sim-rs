use person::{Person, Status, PersonFactory};
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
        let mut factory = PersonFactory::new(); 
        let mut population = Vec::new();
        let mut rng = thread_rng();

        for i in (0..POPULATION_SIZE) {
            let x = rng.gen_range(0u32, ROOM_SIZE);
            let y = rng.gen_range(0u32, ROOM_SIZE);
            let pos = Point2::new(x, y);
            let status;
            if i < INITIAL_INFECTED {
                status = Status::Infectious;
            } else {
                status = Status::Healthy;
            }
            let p = factory.new_person(status, pos);
            population.push(p);
        }

        Simulation {
            population: population,
            rng: rng,
            current_time: 0
        }
    }

    // fn people_within_radius(&self, person: Person) -> Vec<Person> {
    //     self.population
    //         .iter()
    //         .filter(|p| person.distance_to(p) <= INFLUENCE_RADIUS)
    //         .collect()
    // }

    pub fn tick(&mut self) {
        for p in self.population.iter_mut() {
            p.tick(self.current_time, &mut self.rng);
        }

        self.current_time += 1;
    }

}
