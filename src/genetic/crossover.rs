use rand::Rng;

pub fn single_point<T>(parent_one: &Vec<T>, parent_two: &Vec<T>) -> [Vec<T>; 2]
    where T : std::clone::Clone {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0, parent_one.len());
    return [
        [ &parent_one[..index], &parent_two[index..] ].concat(),
        [ &parent_two[..index], &parent_one[index..] ].concat()
    ];
}