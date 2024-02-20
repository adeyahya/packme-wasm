use entity::{Algo, AlgoInput};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
pub mod entity;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize)]
enum Query {
    Version {},
    Pack { input: AlgoInput },
}

#[wasm_bindgen]
pub fn msg(input: &str) -> String {
    let query = serde_json::from_str::<Query>(input);
    if let Ok(query) = query {
        match query {
            Query::Version {} => VERSION.unwrap_or("0.0.0").to_string(),
            Query::Pack { input } => pack(input),
        }
    } else {
        "{\"success\": 0, \"msg\": \"Invalid query\"}".to_string()
    }
}

fn pack(input: AlgoInput) -> String {
    let (containers, items) = input.into_spec();
    let mut algo = Algo { containers, items };
    let result = algo.pack();
    let result_str = serde_json::to_string(&result).unwrap();
    result_str
}
