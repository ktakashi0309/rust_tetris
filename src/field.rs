use super::points::Points;
#[derive(Debug, Clone)]
pub struct Field {
    pub depth: usize,
    pub width: usize,
    pub frame: Points,
}
impl Field {
    pub fn new(depth: usize, width: usize) -> Self {
        let mut frame = Vec::new();
        for i in 0..=depth {
            frame.push((i, 0));
            frame.push((i, width + 1));
        }
        for i in 1..=width {
            frame.push((depth, i));
        }
        Self {
            depth,
            width,
            frame,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Field;
    #[test]
    fn get_frame() {
        let field = Field::new(9, 10);
        assert_eq!(field.frame.len(), 30);
    }
}
