pub trait Phenotype<T: Phenotype<T> + Ord> {
    fn new_random() -> T;
    /*fn to_genotype(&self) -> Vec<u8>;
    fn from_genotype(genotype: &Vec<u8>) -> T;*/
    fn crossover(&self, partner: &T) -> [T; 2];
    fn mutate(&mut self);
}
