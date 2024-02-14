use json_minimal::*;

use super::{Container, ContainerSpec, Error, Item, ItemSpec, Rotation, Vector3};

#[derive(Debug)]
pub struct AlgoInput;

impl AlgoInput {
    fn parse_containers(j: &Json) -> Result<Vec<ContainerSpec>, Error> {
        match j {
            Json::ARRAY(jsons) => {
                let mut containers: Vec<ContainerSpec> = Vec::new();
                for json in jsons.iter() {
                    let id = json.get("id");
                    let qty = json.get("qty");
                    let dim = json.get("dim");
                    match (id, qty, dim) {
                        (
                            Some(Json::STRING(id)),
                            Some(Json::NUMBER(qty)),
                            Some(Json::ARRAY(dim)),
                        ) => match (&dim[0], &dim[1], &dim[3]) {
                            (Json::NUMBER(dim1), Json::NUMBER(dim2), Json::NUMBER(dim3)) => {
                                containers.push(ContainerSpec {
                                    qty: *qty as usize,
                                    spec: Container {
                                        id: id.to_string(),
                                        dim: Vector3::new((*dim1, *dim2, *dim3)),
                                        items: Vec::new(),
                                    },
                                })
                            }
                            _ => return Err(Error::InvalidInput),
                        },
                        _ => return Err(Error::InvalidInput),
                    }
                }

                Ok(containers)
            }
            _ => Err(Error::InvalidInput),
        }
    }

    fn parse_items(j: &Json) -> Result<Vec<ItemSpec>, Error> {
        match j {
            Json::ARRAY(jsons) => {
                let mut items: Vec<ItemSpec> = Vec::new();
                for json in jsons.iter() {
                    let id = json.get("id");
                    let qty = json.get("qty");
                    let dim = json.get("dim");
                    match (id, qty, dim) {
                        (
                            Some(Json::STRING(id)),
                            Some(Json::NUMBER(qty)),
                            Some(Json::ARRAY(dim)),
                        ) => match (&dim[0], &dim[1], &dim[3]) {
                            (Json::NUMBER(dim1), Json::NUMBER(dim2), Json::NUMBER(dim3)) => items
                                .push(ItemSpec {
                                    qty: *qty as usize,
                                    spec: Item {
                                        id: id.to_string(),
                                        dim: Vector3::new((*dim1, *dim2, *dim3)),
                                        pos: Vector3::default(),
                                        rot: Rotation::HLW,
                                    },
                                }),
                            _ => return Err(Error::InvalidInput),
                        },
                        _ => return Err(Error::InvalidInput),
                    }
                }

                Ok(items)
            }
            _ => Err(Error::InvalidInput),
        }
    }

    pub fn parse(input: &str) -> Result<(Vec<ItemSpec>, Vec<ContainerSpec>), Error> {
        Json::parse(input.as_bytes())
            .map(|j| match (j.get("items"), j.get("containers")) {
                (
                    Some(Json::OBJECT {
                        name: _,
                        value: j_items,
                    }),
                    Some(Json::OBJECT {
                        name: _,
                        value: j_containers,
                    }),
                ) => {
                    let items = Self::parse_items(j_items.unbox())?;
                    let containers = Self::parse_containers(j_containers.unbox())?;
                    Ok((items, containers))
                }
                (_, _) => Err(Error::InvalidInput),
            })
            .map_err(|_| Error::ParseError)?
    }
}

#[cfg(test)]
mod tests {
    use super::AlgoInput;

    #[test]
    fn test_parser() {
        let input = String::from(
            r#"{
                "containers": [{"id": "container A", "qty": 1, "dim": [10, 10, 10]}],
                "items": [{"id": "container A", "qty": 1, "dim": [10, 10, 10]}]
            }"#,
        );
        let result = AlgoInput::parse(&input).unwrap();
        // assert_eq!(result.is_err(), false);
    }
}
