use super::points::Points;
use rand::random;

pub fn random_create() -> Points {
    let arr = [
        vec![(0, 1), (1, 1), (2, 1), (3, 1)],
        vec![(0, 1), (1, 1), (2, 1), (2, 2)],
        vec![(0, 1), (0, 2), (1, 1), (2, 1)],
        vec![(0, 2), (0, 3), (1, 1), (1, 2)],
        vec![(0, 1), (0, 2), (1, 2), (1, 3)],
        vec![(0, 1), (0, 2), (1, 1), (1, 2)],
        vec![(0, 1), (0, 2), (0, 3), (1, 2)],
    ];
    let index = random::<u8>() as usize % arr.len();
    arr[index].to_vec()
}
