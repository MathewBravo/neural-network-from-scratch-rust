pub trait Shape {
    fn shape_print(&self) -> String;
}

impl Shape for Vec<f32> {
    fn shape_print(&self) -> String {
        let elements = self.len();
        format!("({}, )\t1D Array, Vector", elements)
    }
}

impl Shape for Vec<Vec<f32>> {
    fn shape_print(&self) -> String {
        let rows = self.len();
        let cols = self[0].len();
        format!("({}, {})\t2D Array, Matrix", rows, cols)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_for_1d() {
        let test_vec = vec![1.0, 2.0, 3.0, 4.0];
        assert_eq!("(4, )\t1D Array, Vector", test_vec.shape_print())
    }

    #[test]
    fn test_shape_for_2d() {
        let test_vec: Vec<Vec<f32>> = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.0, 14.0, 15.0, 16.0],
            vec![17.0, 18.0, 19.0, 20.0],
            vec![21.0, 22.0, 23.0, 24.0],
            vec![25.0, 26.0, 27.0, 28.0],
            vec![29.0, 30.0, 31.0, 32.0],
        ];
        assert_eq!("(8, 4)\t2D Array, Matrix", test_vec.shape_print())
    }
}
