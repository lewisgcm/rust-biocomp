use std::mem;

/**
 * Mutate a genotype using the bit flip technique.
 * This will only flip a single bit for the genotype.
 *
 * # Panics
 * This method will panic if the index or bit are out of range given the input vector to flip.
 */
pub fn flip_bit(index: usize, bit: usize, genotype: &mut Vec<u8>) {
    if genotype.len() < index || bit > (mem::size_of::<u8>() * 8) {
        panic!("Index or bit is out of range for genotype vector")
    }
    genotype[index] ^= 1 << bit;
}

/**
 * Mutate a genotype by flipping all its bits.
 */
pub fn flip_bits(genotype: &mut Vec<u8>) {
    for (_i, elem) in genotype.iter_mut().enumerate() {
        *elem = !*elem;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn flip_bits_test() {
        let mut bits: Vec<u8> = vec![0b00100010];
        flip_bits(&mut bits);
        assert_eq!(bits, [0b11011101]);
    }

    #[test]
    fn flip_bit_test() {
        let mut bits: Vec<u8> = vec![0b00000010];
        flip_bit(0, 6, &mut bits);
        assert_eq!(bits, [0b01000010]);
    }

    #[bench]
    fn flip_bits_benchmark(b: &mut Bencher) {
        let mut bits: Vec<u8> = vec![0b00100010];
        b.iter(|| {
            flip_bits(&mut bits);
        });
    }

    #[bench]
    fn flip_bit_benchmark(b: &mut Bencher) {
        let mut bits: Vec<u8> = vec![0b00100010];
        b.iter(|| {
            flip_bit(0, 6, &mut bits);
        });
    }
}
