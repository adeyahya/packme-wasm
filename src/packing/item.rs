#[derive(Default, Clone, PartialEq, Debug)]
pub struct ItemDimension(pub f64, pub f64, pub f64);

#[derive(Default, Clone, PartialEq, Debug)]
pub struct ItemCoordinate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Item {
    pub quantity: usize,
    pub dim: ItemDimension,
    pub pack_dim: ItemDimension,
    pub coord: ItemCoordinate,
}

impl Item {
    pub fn new(dim1: f64, dim2: f64, dim3: f64, quantity: usize) -> Self {
        Self {
            dim: ItemDimension(dim1, dim2, dim3),
            pack_dim: ItemDimension::default(),
            coord: ItemCoordinate::default(),
            quantity,
        }
    }

    pub fn get_volume(&self) -> f64 {
        self.dim.0 * self.dim.1 * self.dim.2
    }
}
