use rand::Rng;
use rand::FromEntropy;
use rand::rngs::SmallRng;

use super::phenotype::Phenotype;

pub struct Population<T>
where
    T: Phenotype<T> + Ord + Clone,
{
    population: Vec<T>,
    mutation_rate: f32,    // How likley is it that an individual will mutate?
    population_cap: usize, // What is the largest our population can be
}

impl<T> Population<T>
where
    T: Phenotype<T> + Ord + Clone,
{
    pub fn new(mutation_rate: f32, population_cap: usize) -> Population<T> {
        return Population {
            population: Vec::new(),
            mutation_rate: mutation_rate,
            population_cap: population_cap,
        };
    }

    pub fn populate(&mut self) {
        for _i in 0..self.population_cap {
            self.population.push(<T as Phenotype<T>>::new_random());
        }
    }

    pub fn mutate(&mut self) {
        let mut rng = SmallRng::from_entropy();
        for (_i, elem) in self.population.iter_mut().enumerate() {
            if rng.gen_range(0.0, 1.0) >= self.mutation_rate {
                elem.mutate();
            }
        }
    }

    pub fn crossover(&mut self) {
        let mut rng = SmallRng::from_entropy();
        let mut new_children: Vec<T> = Vec::new();
        self.population.sort();

        for (i, parent_one) in self.population.iter().enumerate() {
            let chance = rng.gen_range(0, self.population.len());
            if chance > i {
                for (j, parent_two) in self.population.iter().enumerate() {
                    let second_chance = rng.gen_range(0, self.population.len());

                    if second_chance > j || j == (self.population.len() - 1) {
                        let children = parent_one.crossover(parent_two);
                        new_children.extend_from_slice(&children);
                    }
                }
            }
        }
        self.population.extend_from_slice(&new_children);
    }

    pub fn prune(&mut self) {
        self.population.sort();
        self.population.drain(self.population_cap..);
    }

    pub fn get_result(&mut self, iterations: u32) -> Option<&T> {
        for _i in 0..iterations {
            self.crossover();
            self.prune();
            self.mutate();
        }

        return self.population.iter().max();
    }
}
