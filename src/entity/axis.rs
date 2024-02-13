use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Axis {
    X,
    Y,
    Z,
}
