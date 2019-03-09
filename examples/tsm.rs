extern crate biocomp;
extern crate rand;

use rand::rngs::SmallRng;
use rand::FromEntropy;
use rand::Rng;
use rand::seq::SliceRandom;

use biocomp::genetic::phenotype;
use biocomp::genetic::crossover;
use biocomp::genetic::population::Population;
use std::cmp::Ordering;

/**
 * We will use this point to store locations for our cities
 */
struct City {
    name: &'static str,
    lat: f32,
    long: f32,
}

// List of static cities with lat and long, there are 14! = 1.3*10^12 possibilities
static CITIES: &'static [City] = &[
    City {
        name: "A",
        lat: 0.0,
        long: 6.1,
    },
    City {
        name: "B",
        lat: 1.0,
        long: 8.9,
    },
    City {
        name: "C",
        lat: 1.2,
        long: 2.7,
    },
    City {
        name: "D",
        lat: 2.4,
        long: 0.6,
    },
    City {
        name: "E",
        lat: 0.4,
        long: 0.1,
    },
    City {
        name: "F",
        lat: 1.6,
        long: 3.1,
    },
    City {
        name: "G",
        lat: 6.0,
        long: 8.2,
    },
    City {
        name: "H",
        lat: 0.8,
        long: 1.6,
    },
    City {
        name: "I",
        lat: 1.9,
        long: 1.6,
    },
    City {
        name: "J",
        lat: 1.0,
        long: -1.6,
    },
    City {
        name: "K",
        lat: 5.0,
        long: -2.6,
    },
    City {
        name: "L",
        lat: 9.0,
        long: 9.6,
    },
    City {
        name: "M",
        lat: 23.0,
        long: -23.6,
    },
    City {
        name: "N",
        lat: 3.0,
        long: -0.6,
    },
    City {
        name: "O",
        lat: -4.0,
        long: -0.6,
    },
];

fn distance(a: &City, b: &City) -> f32 {
    let d = (a.lat.powf(2.0) - b.lat.powf(2.0)) + (a.long.powf(2.0) - b.long.powf(2.0));
    return d.abs().sqrt();
}

#[derive(Clone)]
pub struct Solution {
    pub data: Vec<u8>,
    fitness: f32,
}

fn fitness(data: &Vec<u8>) -> f32 {
    let mut total_distance: f32 = 0.0;
    for i in 0..(CITIES.len() - 1) {
        total_distance += distance(&CITIES[data[i] as usize], &CITIES[data[i + 1] as usize]);
    }
    return total_distance;
}

impl Eq for Solution {}

impl Ord for Solution {
    fn cmp(&self, other: &Solution) -> Ordering {
        if self.fitness > other.fitness {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for Solution {
    fn partial_cmp(&self, other: &Solution) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl PartialEq for Solution {
    fn eq(&self, _other: &Solution) -> bool {
        return false;
    }
}

impl phenotype::Phenotype<Solution> for Solution {
    fn new_random() -> Solution {
        let mut slice: Vec<u8> = (0..(CITIES.len() as u8)).collect();
        let mut rng = SmallRng::from_entropy();
        slice.shuffle(&mut rng);
        let fitness = fitness(&slice);
        return Solution {
            data: slice,
            fitness: fitness,
        };
    }
    
    fn crossover(&self, partner: &Solution) -> [Solution; 2] {
        let mut rng = SmallRng::from_entropy();

        let index = rng.gen_range(0, self.data.len());
        let children = crossover::unique_single_point(index, &self.data, &partner.data);

        return [
            Solution {
                data: children[0].clone(),
                fitness: fitness(&children[0]),
            },
            Solution {
                data: children[1].clone(),
                fitness: fitness(&children[1]),
            },
        ];
    }

    fn mutate(&mut self) {
        let mut rng = SmallRng::from_entropy();
        self.data.shuffle(&mut rng);
    }
}

fn main() {
    let mut population: Population<Solution> = Population::new(0.05, 500);
    population.populate();
    let solution = population.get_result(2000);
    for i in solution.data.clone() {
        print!("{} -> ", CITIES[i as usize].name);
    }
    print!("END\nfitness: {:?} \n", solution.fitness);
}
