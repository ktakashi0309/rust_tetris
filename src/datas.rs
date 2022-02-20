use fn_block::IntoSome;
use itertools::Itertools;

use crate::points::get_routing;

use super::field::Field;
use super::points::Point;
use super::points::Points;

#[derive(Debug, Clone)]
pub struct Datas {
    fixed: Points,
    float: Points,
    field: Field,
}

impl Datas {
    pub fn new(field: Field) -> Self {
        Self {
            fixed: Vec::new(),
            float: Vec::new(),
            field: field,
        }
    }
    pub fn clear(&mut self) -> &mut Self {
        self.fixed = Vec::new();
        self.float = Vec::new();
        self
    }

    pub fn get_fixed(&self) -> &Points {
        &self.fixed
    }
    pub fn get_float(&self) -> &Points {
        &self.float
    }
    pub fn get_frame(&self) -> &Points {
        &self.field.frame
    }
    pub fn get_width(&self) -> usize {
        self.field.width
    }
    pub fn get_depth(&self) -> usize {
        self.field.depth
    }
    fn calc_line(&mut self) -> &mut Self {
        self
    }
    pub fn do_routing(&mut self) -> &mut Self {
        if self.can_routing() {
            self.float = get_routing(&self.float).unwrap();
            self
        } else {
            self
        }
    }
    fn can_routing(&self) -> bool {
        let routed = get_routing(&self.float);
        if routed == None {
            return false;
        };
        if self
            .fixed
            .iter()
            .merge(self.field.frame.iter())
            .cartesian_product(routed.unwrap().iter())
            .filter(|x| x.0 == x.1)
            .count()
            != 0
        {
            return false;
        }
        return true;
    }
    fn can_down(&self) -> bool {
        if self.float.len() == 0 {
            return false;
        };
        self.float
            .iter()
            .filter(|(i, j)| {
                self.fixed
                    .iter()
                    .fold(false, |acc, k| acc || (*i + 1, *j) == *k)
                    || self
                        .field
                        .frame
                        .iter()
                        .fold(false, |acc, k| acc || (*i + 1, *j) == *k)
            })
            .count()
            == 0
    }
    fn can_left(&self) -> bool {
        self.float
            .iter()
            .filter(|(i, j)| {
                self.fixed
                    .iter()
                    .fold(false, |acc, k| acc || (*i, *j - 1) == *k)
                    || self
                        .field
                        .frame
                        .iter()
                        .fold(false, |acc, k| acc || (*i, *j - 1) == *k)
            })
            .count()
            == 0
    }
    fn can_right(&self) -> bool {
        self.float
            .iter()
            .filter(|(i, j)| {
                self.fixed
                    .iter()
                    .fold(false, |acc, k| acc || (*i, *j + 1) == *k)
                    || self
                        .field
                        .frame
                        .iter()
                        .fold(false, |acc, k| acc || (*i, *j + 1) == *k)
            })
            .count()
            == 0
    }
    fn to_fixed(&mut self) -> &mut Self {
        self.fixed.append(&mut self.float);
        self
    }
    pub fn down_action(&mut self) -> &mut Self {
        if !self.can_down() {
            self.to_fixed();
            self.calc_line();
            return self;
        }
        self.float = self.float.iter().map(|&(i, j)| (i + 1, j)).collect_vec();
        self
    }
    pub fn left_action(&mut self) -> &mut Self {
        if !self.can_left() {
            return self;
        }
        self.float = self.float.iter().map(|&(i, j)| (i, j - 1)).collect_vec();
        self
    }
    pub fn right_action(&mut self) -> &mut Self {
        if !self.can_right() {
            return self;
        }
        self.float = self.float.iter().map(|&(i, j)| (i, j + 1)).collect_vec();
        self
    }
    pub fn is_dead(&self) -> bool {
        let count = self
            .float
            .iter()
            .filter(|&x| self.fixed.iter().fold(true, |acc, y| acc && x != y))
            .count();
        count == 0
    }
    pub fn create_float(&mut self, mut new: Points) -> &mut Self {
        self.float.append(&mut new);
        self
    }
    pub fn exist_float(&self) -> bool {
        self.float.len() != 0
    }
    pub fn get_deletable_lines(&self) -> Vec<usize> {
        (0..self.field.depth)
            .into_iter()
            .filter(|i| {
                self.fixed.iter().filter(|(x, _)| x == i).unique().count() == self.field.width
            })
            .collect_vec()
    }

    fn calc_down_lines(delete_lines: &Vec<usize>, point: &Point) -> Option<usize> {
        if delete_lines.iter().filter(|&x| x == &point.0).count() > 0 {
            return None;
        };
        delete_lines
            .iter()
            .fold(0, |acc, x| if x > &point.0 { acc + 1 } else { acc })
            .into_some()
    }
    pub fn delete_lines(&mut self) -> &mut Self {
        let deletable_lines = self.get_deletable_lines();
        self.fixed = self
            .fixed
            .drain(..)
            .filter_map(|x| Self::calc_down_lines(&deletable_lines, &x).map(|y| (x.0 + y, x.1)))
            .collect_vec();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::Datas;
    use super::Field;

    #[test]
    fn is_dead_true() {
        let mut datas = Datas::new(Field::new(4, 4));
        datas.fixed.push((0, 0));
        datas.fixed.push((1, 0));
        datas.fixed.push((1, 1));
        datas.float.push((1, 0));
        assert_eq!(datas.is_dead(), true);
    }
    #[test]
    fn is_dead_false() {
        let mut datas = Datas::new(Field::new(4, 4));
        datas.fixed.push((0, 0));
        datas.fixed.push((1, 0));
        datas.fixed.push((1, 1));
        datas.float.push((2, 2));
        assert_eq!(datas.is_dead(), false);
    }
    #[test]
    fn delete_lines() {
        let mut datas = Datas::new(Field::new(2, 2));
        datas.fixed.push((0, 0));
        datas.fixed.push((1, 0));
        datas.fixed.push((1, 1));
        assert_eq!(datas.delete_lines().fixed.len(), 1);
    }
    #[test]
    fn get_deletable_line() {
        let mut datas = Datas::new(Field::new(2, 2));
        datas.fixed.push((0, 0));
        datas.fixed.push((0, 1));
        assert_eq!(datas.get_deletable_lines().len(), 1);
    }
    #[test]
    fn can_down_true() {
        let mut datas = Datas::new(Field::new(10, 10));
        datas.float.push((1, 1));
        assert_eq!(datas.can_down(), true);
    }
    #[test]
    fn can_down_false() {
        let mut datas = Datas::new(Field::new(10, 10));
        datas.float.push((9, 1));
        assert_eq!(datas.can_down(), false);
    }
    #[test]
    fn can_down_none() {
        let datas = Datas::new(Field::new(10, 10));
        assert_eq!(datas.can_down(), false);
    }
    #[test]
    fn can_down_false_fixed() {
        let mut datas = Datas::new(Field::new(10, 10));
        datas.fixed.push((3, 2));
        datas.float.push((2, 2));
        assert_eq!(datas.can_down(), false);
    }
    #[test]
    fn can_right_false() {
        let mut datas = Datas::new(Field::new(10, 10));
        datas.float.push((1, 10));
        assert_eq!(datas.can_right(), false);
    }
    #[test]
    fn can_right_true() {
        let mut datas = Datas::new(Field::new(10, 10));
        datas.float.push((1, 7));
        assert_eq!(datas.can_right(), true);
    }
    #[test]
    fn can_left_false() {
        let mut datas = Datas::new(Field::new(10, 10));
        datas.float.push((1, 1));
        assert_eq!(datas.can_left(), false);
    }
    #[test]
    fn can_left_true() {
        let mut datas = Datas::new(Field::new(10, 10));
        datas.float.push((1, 2));
        assert_eq!(datas.can_left(), true);
    }
    #[test]
    fn to_fixed() {
        let mut datas = Datas::new(Field::new(10, 10));
        datas.float.push((1, 2));
        datas.to_fixed();
        assert_eq!(datas.fixed, vec![(1, 2)]);
        assert_eq!(datas.float, vec![]);
    }
}
