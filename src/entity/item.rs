use super::{Axis, Rotation, Vector3};

#[derive(Clone, Debug)]
pub struct Item {
    pub id: String,
    pub dim: Vector3,
    pub pos: Vector3,
    pub rot: Rotation,
}

impl Item {
    pub fn new(id: String, length: f64, width: f64, height: f64) -> Self {
        Self {
            id,
            dim: Vector3::new((length, width, height)),
            pos: Vector3::default(),
            rot: Rotation::LWH,
        }
    }
    pub fn dimensions(&self) -> Vector3 {
        match self.rot {
            Rotation::LWH => Vector3::new((self.dim.length, self.dim.width, self.dim.height)),
            Rotation::WLH => Vector3::new((self.dim.width, self.dim.length, self.dim.height)),
            Rotation::WHL => Vector3::new((self.dim.width, self.dim.height, self.dim.length)),
            Rotation::HLW => Vector3::new((self.dim.height, self.dim.length, self.dim.width)),
            Rotation::HWL => Vector3::new((self.dim.height, self.dim.width, self.dim.length)),
            Rotation::LHW => Vector3::new((self.dim.length, self.dim.height, self.dim.width)),
        }
    }

    pub fn collision(item_a: &Item, item_b: &Item, ax: Axis, ay: Axis) -> bool {
        let dim1 = item_a.dimensions();
        let dim2 = item_b.dimensions();

        let center_x1 = item_a.pos.get_by_axis(&ax) + dim1.get_by_axis(&ax) * 0.5;
        let center_y1 = item_a.pos.get_by_axis(&ay) + dim1.get_by_axis(&ay) * 0.5;

        let center_x2 = item_b.pos.get_by_axis(&ax) + dim2.get_by_axis(&ax) * 0.5;
        let center_y2 = item_b.pos.get_by_axis(&ay) + dim2.get_by_axis(&ay) * 0.5;

        let x = center_x1.max(center_x2) - center_x1.min(center_x2);
        let y = center_y1.max(center_y2) - center_y1.min(center_y2);

        x < (dim1.get_by_axis(&ax) + dim2.get_by_axis(&ax)) * 0.5
            && y < (dim1.get_by_axis(&ay) + dim2.get_by_axis(&ay)) * 0.5
    }

    pub fn collisions(&self, item_b: &Item) -> bool {
        Item::collision(self, item_b, Axis::X, Axis::Y)
            && Item::collision(self, item_b, Axis::Y, Axis::Z)
            && Item::collision(self, item_b, Axis::X, Axis::Z)
    }
}

pub struct ItemSpec {
    pub spec: Item,
    pub qty: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        let item_1 = Item::new("item_1".into(), 10.0, 10.0, 30.0);
        let mut item_2 = Item::new("item_2".into(), 10.0, 10.0, 30.0);
        assert_eq!(item_1.collisions(&item_2), true);
        item_2.pos.length = 10.0;
        assert_eq!(item_1.collisions(&item_2), false);
    }

    #[test]
    fn test_rotate_lwh() {
        let mut item_1 = Item::new("item_1".into(), 10.0, 20.0, 30.0);
        item_1.rot = Rotation::LWH;
        let dim = item_1.dimensions();
        let expected_lwh = Vector3::new((10.0, 20.0, 30.0));
        assert_eq!(dim, expected_lwh);
    }

    #[test]
    fn test_rotate_whl() {
        let mut item_1 = Item::new("item_1".into(), 10.0, 20.0, 30.0);
        item_1.rot = Rotation::WHL;
        let dim = item_1.dimensions();
        let expected_whl = Vector3::new((20.0, 30.0, 10.0));
        assert_eq!(dim, expected_whl);
    }

    #[test]
    fn test_rotate_wlh() {
        let mut item_1 = Item::new("item_1".into(), 10.0, 20.0, 30.0);
        item_1.rot = Rotation::WLH;
        let dim = item_1.dimensions();
        let expected_wlh = Vector3::new((20.0, 10.0, 30.0));
        assert_eq!(dim, expected_wlh);
    }

    #[test]
    fn test_rotate_hlw() {
        let mut item_1 = Item::new("item_1".into(), 10.0, 20.0, 30.0);
        item_1.rot = Rotation::HLW;
        let dim = item_1.dimensions();
        let expected_hlw = Vector3::new((30.0, 10.0, 20.0));
        assert_eq!(dim, expected_hlw);
    }

    #[test]
    fn test_rotate_hwl() {
        let mut item_1 = Item::new("item_1".into(), 10.0, 20.0, 30.0);
        item_1.rot = Rotation::HWL;
        let dim = item_1.dimensions();
        let expected_hwl = Vector3::new((30.0, 20.0, 10.0));
        assert_eq!(dim, expected_hwl);
    }

    #[test]
    fn test_rotate_lhw() {
        let mut item_1 = Item::new("item_1".into(), 10.0, 20.0, 30.0);
        item_1.rot = Rotation::LHW;
        let dim = item_1.dimensions();
        let expected_lhw = Vector3::new((10.0, 30.0, 20.0));
        assert_eq!(dim, expected_lhw);
    }
}
