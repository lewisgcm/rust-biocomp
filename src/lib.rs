mod genetic;
use self::genetic::phenotype;
use self::genetic::crossover;
use self::genetic::mutation;
use std::cmp::Ordering;

#[derive(Eq, Copy, Clone)]
pub struct Solution {
    pub data: [u8; 32]
}

impl Solution {
    fn fitness(&self) -> f32 {
        return 0.5;
    }
}

impl Ord for Solution {
    fn cmp(&self, other: &Solution) -> Ordering {
        self.fitness().partial_cmp( &other.fitness() ).unwrap()
    }
}

impl PartialOrd for Solution {
    fn partial_cmp(&self, other: &Solution) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Solution {
    fn eq(&self, other: &Solution) -> bool {
        return self.fitness() == other.fitness();
    }
}

impl phenotype::Phenotype<Solution> for Solution {
    fn new_random() -> Solution {
        return Solution {
            data: [0; 32]
        }
    }

    fn to_genotype(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity( 10 );
        data.extend_from_slice( &self.data );
        return data;
    }

    fn from_genotype( genotype: &Vec<u8> ) -> Solution {
        return Solution {
            data: [0; 32]
        }
    }

    fn crossover(&self, partner: &Solution) -> [Solution; 2] {
        let children = crossover::single_point( &self.to_genotype(), &partner.to_genotype() );
        return [
            Solution::from_genotype( &children[0] ),
            Solution::from_genotype( &children[1] )
        ];
    }

    fn mutate(&mut self) {
        mutation::flip_bit(&mut self.data);
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
