/**
 * Crosses over two parents to produce two children.
 *
 * # Panics
 * This method will panic if the supplied index is out of rang for either vector
 */
pub fn single_point<T>(point: usize, parent_one: &Vec<T>, parent_two: &Vec<T>) -> [Vec<T>; 2]
where
    T: std::clone::Clone,
{
    if parent_one.len() <= point || parent_two.len() <= point {
        panic!("supplied index is bigger than supplied parent vectors")
    }
    return [
        [&parent_one[..point], &parent_two[point..]].concat(),
        [&parent_two[..point], &parent_one[point..]].concat(),
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_point_test() {
        let parent_one: Vec<u8> = vec![1, 2, 3];
        let parent_two: Vec<u8> = vec![4, 5, 6];
        let children = single_point(1, &parent_one, &parent_two);

        assert_eq!(children[0], [1, 5, 6]);
        assert_eq!(children[1], [4, 2, 3]);
    }

    #[test]
    fn single_point_different_lengths_test() {
        let parent_one: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let parent_two: Vec<u8> = vec![7, 8, 9];
        let children = single_point(2, &parent_one, &parent_two);

        assert_eq!(children[0], [1, 2, 9]);
        assert_eq!(children[1], [7, 8, 3, 4, 5, 6]);
    }
}
