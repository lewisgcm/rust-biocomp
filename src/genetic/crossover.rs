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

/**
 * Crosses over two parents to produce two children, this is suitable for unique genomes.
 *
 * # Panics
 * This method will panic if the supplied index is out of rang for either vector
 */
pub fn unique_single_point<T>(point: usize, parent_one: &Vec<T>, parent_two: &Vec<T>) -> [Vec<T>; 2]
where
    T: std::clone::Clone,
    T: std::cmp::PartialEq,
{
    if parent_one.len() <= point || parent_two.len() <= point {
        panic!("supplied index is bigger than supplied parent vectors")
    }

    let mut child_one = parent_one[..point].to_vec();
    for i in 0..parent_two.len() {
        if !child_one.contains(&parent_two[i]) {
            child_one.push(parent_two[i].clone());
        }
    }

    let mut child_two = parent_two[..point].to_vec();
    for i in 0..parent_one.len() {
        if !child_two.contains(&parent_one[i]) {
            child_two.push(parent_one[i].clone());
        }
    }

    return [
        child_one,
        child_two,
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

    #[test]
    fn unique_single_point_test() {
        let parent_one: Vec<u8> = vec![1, 2, 3, 4];
        let parent_two: Vec<u8> = vec![4, 3, 2, 1];
        let children = unique_single_point(2, &parent_one, &parent_two);

        assert_eq!(children[0], [1, 2, 4, 3]);
        assert_eq!(children[1], [4, 3, 1, 2]);
    }
}
