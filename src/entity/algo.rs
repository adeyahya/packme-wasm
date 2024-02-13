use std::cmp;

use super::{Axis, Container, ContainerSpec, Item, ItemSpec, Rotation, Vector3};

pub struct Algo {
    pub items: Vec<ItemSpec>,
    pub containers: Vec<ContainerSpec>,
}

#[derive(Debug)]
pub struct AlgoResult {
    pub unpacked_items: Vec<Item>,
    pub containers: Vec<Container>,
}

impl Algo {
    pub fn pack(&mut self) -> AlgoResult {
        self.containers.sort_by(|a, b| {
            if a.spec.dim.volume() < b.spec.dim.volume() {
                cmp::Ordering::Less
            } else if a.spec.dim.volume() > b.spec.dim.volume() {
                cmp::Ordering::Greater
            } else {
                cmp::Ordering::Equal
            }
        });

        self.items.sort_by(|a, b| {
            if a.spec.dim.volume() < b.spec.dim.volume() {
                cmp::Ordering::Less
            } else if a.spec.dim.volume() > b.spec.dim.volume() {
                cmp::Ordering::Greater
            } else {
                cmp::Ordering::Equal
            }
        });

        let mut containers: Vec<Container> = Vec::new();
        let mut unpacked_items: Vec<Item> = Vec::new();

        for item_spec in self.items.iter_mut() {
            for _ in 0..item_spec.qty {
                let mut new_item = item_spec.spec.clone();
                let mut is_packed = false;

                for container in containers.iter_mut() {
                    is_packed = Algo::pack_item(&mut new_item, container);
                    if is_packed {
                        break;
                    }
                }

                if !is_packed {
                    for c in self.containers.iter_mut() {
                        if c.qty > 0 {
                            let mut new_container = c.spec.clone();
                            is_packed = Algo::pack_item(&mut new_item, &mut new_container);
                            if is_packed {
                                c.qty = c.qty - 1;
                                containers.push(new_container);
                                break;
                            }
                        }
                    }

                    if !is_packed {
                        unpacked_items.push(new_item);
                    }
                }
            }
        }

        AlgoResult {
            containers,
            unpacked_items,
        }
    }

    fn pack_item(item: &mut Item, c: &mut Container) -> bool {
        if c.items.len() < 1 {
            return Algo::pack_to_box(item, c, Vector3::default());
        }

        let axis = [Axis::X, Axis::Y, Axis::Z];
        for axis in axis {
            for x in 0..c.items.len() {
                let pivot =
                    Vector3::compute_pivot(&axis, &c.items[x].pos, &c.items[x].dimensions());
                if Algo::pack_to_box(item, c, pivot) {
                    return true;
                }
            }
        }
        false
    }

    fn pack_to_box(item: &mut Item, c: &mut Container, pivot: Vector3) -> bool {
        let mut is_packed = false;
        item.pos = pivot.clone();

        let rotations = [
            Rotation::LWH,
            Rotation::WHL,
            Rotation::WLH,
            Rotation::HLW,
            Rotation::HWL,
            Rotation::LHW,
        ];
        for rot in rotations {
            item.rot = rot;
            let idims = item.dimensions();

            if c.dim.length < &pivot.length + idims.length
                || c.dim.width < &pivot.width + idims.width
                || c.dim.height < &pivot.height + idims.height
            {
                continue;
            }

            is_packed = true;

            for item_i in &c.items {
                if item.collisions(item_i) {
                    is_packed = false;
                    break;
                }
            }

            if is_packed {
                c.items.push(item.clone());
            }

            break;
        }

        if !is_packed {
            item.pos = Vector3::default();
        }

        is_packed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packing_test() {
        let mut algo = Algo {
            containers: vec![
                ContainerSpec {
                    qty: 1,
                    spec: Container::new("Box A".into(), Vector3::new((30.0, 30.0, 30.0))),
                },
                ContainerSpec {
                    qty: 1,
                    spec: Container::new("Box B".into(), Vector3::new((5.0, 5.0, 40.0))),
                },
                ContainerSpec {
                    qty: 1,
                    spec: Container::new("Box C".into(), Vector3::new((20.0, 20.0, 30.0))),
                },
            ],
            items: vec![
                ItemSpec {
                    qty: 17,
                    spec: Item::new("Item A1".into(), 10.0, 10.0, 30.0),
                },
                ItemSpec {
                    qty: 1,
                    spec: Item::new("Item A2".into(), 10.0, 10.0, 30.0),
                },
                ItemSpec {
                    qty: 1,
                    spec: Item::new("Tall Item".into(), 5.0, 39.5, 5.0),
                },
                ItemSpec {
                    qty: 1,
                    spec: Item::new("Large Item".into(), 50.0, 50.0, 100.0),
                },
            ],
        };

        let result = algo.pack();
        assert_eq!(result.containers.len(), 3);
        assert_eq!(result.containers[0].items.len(), 1);
        assert_eq!(result.containers[1].items.len(), 4);
        assert_eq!(result.containers[2].items.len(), 9);
        assert_eq!(result.unpacked_items.len(), 6);
    }
}
