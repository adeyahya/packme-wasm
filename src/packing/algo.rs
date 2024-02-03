use super::{Container, Item};

#[derive(Default)]
struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

struct Layer {
    pub layer_dim: f64,
    pub layer_eval: f64,
}

pub struct EbAfit<'a> {
    pub container: &'a Container,
    item_list: Vec<Item>,
    layer_list: Vec<Layer>,
    orientation_variant: OrientationVariant<'a>,
    orientation: Vector3,
    pub temp: f64,
    // sum of volume of all item / box
    pub total_box_vol: f64,
    // current box index that being evaluated
    bn: usize,

    same: bool,
}

impl<'a> EbAfit<'a> {
    pub fn from_input(container: &'a Container, item_list: &'a Vec<Item>) -> Self {
        let mut orientation_variant = OrientationVariant::from_container(container);
        let orientation = orientation_variant.next().unwrap();

        // flatten item list and filter out if the quantity are 0
        let mut computed_item_list = Vec::new();
        for item in item_list.iter() {
            if item.quantity == 0 {
                continue;
            };

            for _ in 0..item.quantity {
                let mut item = item.clone();
                item.quantity = 1;
                computed_item_list.push(item);
            }
        }

        let total_box_vol = computed_item_list
            .iter()
            .map(|n| n.get_volume())
            .reduce(|acc, n| acc + n)
            .unwrap_or(0.0);

        Self {
            container,
            item_list: computed_item_list,
            layer_list: Vec::new(),
            orientation_variant,
            orientation,
            total_box_vol,
            same: false,
            temp: 0.0,
            bn: 0,
        }
    }

    pub fn pack(&mut self) {
        while self.next().is_some() {}
    }

    fn get_current_item(&self) -> Option<Item> {
        if let Some(item) = self.item_list.get(self.bn) {
            Some(item.clone())
        } else {
            None
        }
    }

    fn compute_candit_layer(&mut self, item: &Item) {
        let mut exdim: f64;
        let mut dimdif: f64;
        let mut dimen2: f64;
        let mut dimen3: f64;
        let mut layer_eval = 0.0;
        let py = self.orientation.y;
        let px = self.orientation.x;
        let pz = self.orientation.z;

        for y in 1..=3 {
            match y {
                1 => {
                    exdim = item.dim.0;
                    dimen2 = item.dim.1;
                    dimen3 = item.dim.2;
                }
                2 => {
                    exdim = item.dim.1;
                    dimen2 = item.dim.0;
                    dimen3 = item.dim.2;
                }
                3 => {
                    exdim = item.dim.2;
                    dimen2 = item.dim.0;
                    dimen3 = item.dim.1;
                }
                _ => unreachable!(),
            }

            if exdim > py || (dimen2 > px || dimen3 > pz) && (dimen3 > px || dimen2 > pz) {
                continue;
            }

            self.same = false;

            for k in 1..=self.layer_list.len() {
                if let Some(layer) = self.layer_list.get(k) {
                    if exdim == layer.layer_dim {
                        self.same = true;
                        continue;
                    }
                }
            }

            if self.same {
                continue;
            }

            for z in 1..=self.item_list.len() {
                if let Some(compared_item) = self.item_list.get(z) {
                    if item != compared_item {
                        dimdif = (exdim - compared_item.dim.0).abs().min(
                            (exdim - compared_item.dim.1)
                                .abs()
                                .min((exdim - compared_item.dim.2).abs()),
                        );
                        layer_eval += dimdif;
                    }
                }
            }

            self.layer_list.push(Layer {
                layer_eval,
                layer_dim: exdim,
            });
        }
    }
}

// implement iterator for the computation loop
// this is important for visualization
impl<'a> Iterator for EbAfit<'a> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.get_current_item();
        if item.is_some() {
            self.compute_candit_layer(&item.unwrap());
            self.bn += 1;
            Some(())
        } else {
            // eof loop for current orientation
            // proceed to next orientation if available

            // sets the evaluation of the first layer in the list to -1
            if let Some(layer) = self.layer_list.get_mut(0) {
                layer.layer_eval = -1.0;
            }
            // sort layer list
            self.layer_list.sort_by(|a, b| {
                if a.layer_eval < b.layer_eval {
                    std::cmp::Ordering::Less
                } else if a.layer_eval > b.layer_eval {
                    std::cmp::Ordering::Greater
                } else {
                    a.layer_dim
                        .partial_cmp(&b.layer_dim)
                        .unwrap_or(std::cmp::Ordering::Equal)
                }
            });
            if let Some(orientation) = self.orientation_variant.next() {
                self.bn = 0;
                self.orientation = orientation;
                self.next()
            } else {
                None
            }
        }
    }
}

// iterator that represent 6 possible different orientation
// of the container
struct OrientationVariant<'a> {
    current_variant: usize,
    container: &'a Container,
}

impl<'a> OrientationVariant<'a> {
    pub fn from_container(container: &'a Container) -> Self {
        Self {
            current_variant: 0,
            container,
        }
    }
}

impl<'a> Iterator for OrientationVariant<'a> {
    type Item = Vector3;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_variant {
            0 => Some(Vector3 {
                x: self.container.width,
                y: self.container.height,
                z: self.container.length,
            }),
            1 => Some(Vector3 {
                x: self.container.length,
                y: self.container.height,
                z: self.container.width,
            }),
            2 => Some(Vector3 {
                x: self.container.length,
                y: self.container.width,
                z: self.container.height,
            }),
            3 => Some(Vector3 {
                x: self.container.height,
                y: self.container.width,
                z: self.container.length,
            }),
            4 => Some(Vector3 {
                x: self.container.width,
                y: self.container.length,
                z: self.container.height,
            }),
            5 => Some(Vector3 {
                x: self.container.height,
                y: self.container.length,
                z: self.container.width,
            }),
            _ => None,
        }
    }
}
