use entity::{Algo, AlgoInput};
use wasm_bindgen::prelude::*;
pub mod entity;

#[wasm_bindgen]
pub unsafe fn pack(input: &str) -> String {
    let input: AlgoInput = serde_json::from_str(&input).unwrap();
    let (containers, items) = input.into_spec();
    let mut algo = Algo { containers, items };
    let result = algo.pack();
    let result_str = serde_json::to_string(&result).unwrap();
    result_str
}
