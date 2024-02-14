use super::{Item, Vector3};

#[derive(Clone, Debug)]
pub struct Container {
    pub id: String,
    pub dim: Vector3,
    pub items: Vec<Item>,
}

impl Container {
    pub fn new(id: String, dim: Vector3) -> Self {
        Self {
            id,
            dim,
            items: Vec::new(),
        }
    }
}

pub struct ContainerSpec {
    pub spec: Container,
    pub qty: usize,
}
