use serde::{Deserialize, Serialize};

use super::{Container, ContainerSpec, Item, ItemSpec, Vector3};

#[derive(Debug, Serialize, Deserialize)]
pub struct AlgoItemInput {
    id: String,
    qty: usize,
    dim: [f64; 3],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlgoInput {
    items: Vec<AlgoItemInput>,
    containers: Vec<AlgoItemInput>,
}

impl AlgoInput {
    pub fn into_spec(&self) -> (Vec<ContainerSpec>, Vec<ItemSpec>) {
        let containers: Vec<ContainerSpec> = self
            .containers
            .iter()
            .map(|c| ContainerSpec {
                qty: c.qty,
                spec: Container::new(c.id.clone(), Vector3::new((c.dim[0], c.dim[1], c.dim[2]))),
            })
            .collect();

        let items: Vec<ItemSpec> = self
            .items
            .iter()
            .map(|i| ItemSpec {
                qty: i.qty,
                spec: Item::new(i.id.clone(), i.dim[0], i.dim[1], i.dim[2]),
            })
            .collect();
        (containers, items)
    }
}
