pub trait Phenotype<T: Phenotype<T> + Ord> {
    fn new_random() -> T;
    fn crossover(&self, partner: &T) -> [T; 2];
    fn mutate(&mut self);
}
