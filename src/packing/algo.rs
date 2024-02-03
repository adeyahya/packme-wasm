use super::{Container, Item};

#[derive(Default)]
struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct EbAfit<'a> {
    container: &'a Container,
    item_list: &'a Vec<Item>,
    orientation_variant: OrientationVariant<'a>,
    orientation: Vector3,
    temp: f64,
    // sum of volume of all item / box
    total_box_vol: f64,
    // current box index that being evaluated
    bn: usize,
}

impl<'a> EbAfit<'a> {
    pub fn from_input(container: &'a Container, item_list: &'a Vec<Item>) -> Self {
        let mut orientation_variant = OrientationVariant::from_container(container);
        let orientation = orientation_variant.next().unwrap();
        let total_box_vol = item_list
            .iter()
            .map(|n| n.get_volume())
            .reduce(|acc, n| acc + n)
            .unwrap_or(0.0);

        Self {
            container,
            item_list,
            orientation_variant,
            orientation,
            total_box_vol,
            temp: 0.0,
            bn: 0,
        }
    }

    pub fn pack(&mut self) {
        while self.next().is_some() {}
    }

    fn compute_item(&mut self, item: &Item) {}
}

// implement iterator for the computation loop
// this is important for visualization
impl<'a> Iterator for EbAfit<'a> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.item_list.get(self.bn) {
            self.compute_item(item);
            self.bn += 1;
            Some(())
        } else {
            // eof loop for current orientation
            // proceed to next orientation if available
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
