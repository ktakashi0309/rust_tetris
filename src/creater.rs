use super::points::Points;
use rand::random;

use super::eventloop::Creater;

pub struct RandomCreater {}

impl RandomCreater {
    pub fn get_points(index: usize) -> Points {
        let arr = [
            vec![(0, 1), (1, 1), (2, 1), (3, 1)],
            vec![(0, 1), (1, 1), (2, 1), (2, 2)],
            vec![(0, 1), (0, 2), (1, 1), (2, 1)],
            vec![(0, 2), (0, 3), (1, 1), (1, 2)],
            vec![(0, 1), (0, 2), (1, 2), (1, 3)],
            vec![(0, 1), (0, 2), (1, 1), (1, 2)],
            vec![(0, 1), (0, 2), (0, 3), (1, 2)],
        ];
        arr[index % arr.len()].to_vec()
    }

    pub fn random_create() -> Points {
        let index = random::<u8>() as usize;
        Self::get_points(index)
    }
}

impl Creater for RandomCreater {
    fn create(&self) -> Points {
        Self::random_create()
    }
}
