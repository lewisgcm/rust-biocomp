#![feature(test)]
extern crate biocomp;
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    use biocomp::genetic::crossover;
    use biocomp::genetic::mutation;

    #[bench]
    fn single_point_bench(b: &mut Bencher) {
        let parent_one: Vec<u8> = vec![1, 2, 3];
        let parent_two: Vec<u8> = vec![4, 5, 6];
        b.iter(|| {
            crossover::single_point(1, &parent_one, &parent_two);
        });
    }

    #[bench]
    fn flip_bits_benchmark(b: &mut Bencher) {
        let mut bits: Vec<u8> = vec![0b00100010];
        b.iter(|| {
            mutation::flip_bits(&mut bits);
        });
    }

    #[bench]
    fn flip_bit_benchmark(b: &mut Bencher) {
        let mut bits: Vec<u8> = vec![0b00100010];
        b.iter(|| {
            mutation::flip_bit(0, 6, &mut bits);
        });
    }
}
