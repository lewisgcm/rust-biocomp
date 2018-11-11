use std::mem;
use rand::Rng;

/**
 * Mutate a genotype using the bit flip technique.
 * This will only flip a single bit for the genotype.
 */
pub fn flip_bit(genotype: &mut [u8]) {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0, genotype.len());
    let bit = rng.gen_range(0, mem::size_of::<u8>());
    genotype[ index ] ^= 1<<bit;
}

/**
 * Mutate a genotype by flipping all its bits.
 */
pub fn flip_bits(genotype: &mut [u8]) {
    for (i, elem) in genotype.iter_mut().enumerate() {
        *elem = !*elem;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flip_bits_test() {
        let mut bits: [u8; 1] = [0b00100010];
        flip_bits( &mut bits );
        assert_eq!( bits, [0b11011101] );
    }
}
