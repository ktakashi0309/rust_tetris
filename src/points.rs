pub type Point = (usize, usize);
pub type Points = Vec<Point>;
use itertools::Itertools;

fn routing_shaft(points: &Points, shaft: Point) -> Option<Points> {
    let new_points = points
        .clone()
        .into_iter()
        .filter_map(|(x, y)| {
            let new_x = shaft.0 as isize + (shaft.1 as isize - y as isize);
            let new_y = shaft.1 as isize + (x as isize - shaft.0 as isize);
            if new_x < 0 || new_y < 0 {
                None
            } else {
                Some((new_x as usize, new_y as usize))
            }
        })
        .collect_vec();
    if new_points.len() == points.len() {
        Some(new_points)
    } else {
        None
    }
}

fn get_gravity(points: &Points) -> Point {
    let length = points.len();
    let sum = points
        .iter()
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
    (
        ((sum.0 as f64) / length as f64).round() as usize,
        ((sum.1 as f64) / length as f64).round() as usize,
    )
}

pub fn get_routing(points: &Points) -> Option<Points> {
    routing_shaft(points, get_gravity(points))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn routing_shaft_test() {
        let mut points = Vec::new();
        points.push((3, 3));
        points.push((3, 4));
        points.push((3, 5));
        points.push((2, 4));
        let mut points2 = routing_shaft(&points, (3, 4)).unwrap();
        points2.sort_unstable();
        assert_eq!(points2[0], (2, 4));
    }
    #[test]
    fn get_gravity_test() {
        let mut points = Vec::new();
        points.push((0, 0));
        points.push((1, 0));
        points.push((1, 1));
        let gravity = get_gravity(&points);
        assert_eq!(gravity, (1, 0));
    }
    #[test]
    fn get_gravity_test2() {
        let mut points = Vec::new();
        points.push((3, 3));
        points.push((3, 4));
        points.push((3, 5));
        let gravity = get_gravity(&points);
        assert_eq!(gravity, (3, 4));
    }
    #[test]
    fn get_routing_test() {
        let mut points = Vec::new();
        points.push((3, 3));
        points.push((3, 4));
        points.push((3, 5));
        let mut routed = get_routing(&points).unwrap();
        routed.sort_unstable();
        assert_eq!(routed[0], (2, 4));
    }
}
