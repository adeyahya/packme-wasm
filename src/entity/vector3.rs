use super::Axis;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Vector3 {
    pub length: f64,
    pub width: f64,
    pub height: f64,
}

impl Vector3 {
    pub fn new(params: (f64, f64, f64)) -> Self {
        Self {
            length: params.0,
            width: params.1,
            height: params.2,
        }
    }
    pub fn volume(&self) -> f64 {
        self.length * self.width * self.height
    }

    pub fn get_by_axis(&self, axis: &Axis) -> f64 {
        match axis {
            Axis::X => self.length,
            Axis::Y => self.width,
            Axis::Z => self.height,
        }
    }

    pub fn compute_pivot(axis: &Axis, pos: &Vector3, dims: &Vector3) -> Vector3 {
        match axis {
            Axis::X => Vector3::new((pos.length + dims.length, pos.width, pos.height)),
            Axis::Y => Vector3::new((pos.length, pos.width + dims.width, pos.height)),
            Axis::Z => Vector3::new((pos.length, pos.width, pos.height + dims.height)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vector3;

    #[test]
    fn test_vector_3() {
        let dim = Vector3::new((10.0, 20.0, 30.0));
        let volume = dim.volume();
        assert_eq!(volume, 6000.0);
    }
}
