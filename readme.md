# packme-wasm

Demo [https://packme.vercel.app](https://packme.vercel.app)

This repository hosts an implementation of [Dube, E., & Kanavathy L. (2006). Optimizing Three-Dimensional Bin Packing Through Simulation.](https://www.researchgate.net/publication/228974015_Optimizing_Three-Dimensional_Bin_Packing_Through_Simulation), written in Rust and distributed using .wasm / WebAssembly.

The creation of this repository serves to demonstrate the concept of a Universal Library using WebAssembly. This concept allows a single binary to be executed within any runtime, host, or programming language. It aims to dispel the common misconception that WebAssembly is solely for web-based applications.

## Bindings

| Language | Upstream                                   | Runtime  |
| -------- | ------------------------------------------ | -------- |
| go       | https://github.com/adeyahya/packme-wasm.go | wasmtime |
| node/js  | https://www.npmjs.com/package/packme-wasm  | built-in |
| python   | TBD                                        | wasmtime |
| ruby     | TBD                                        | wasmtime |
| php      | TBD                                        | wasmer   |

### Todo

- [ ] replace serde with minimal_json

## Credits

- [https://github.com/stevenferrer/packme](https://github.com/stevenferrer/packme)
- [Dube, E., & Kanavathy L. (2006). Optimizing Three-Dimensional Bin Packing Through Simulation.](https://www.researchgate.net/publication/228974015_Optimizing_Three-Dimensional_Bin_Packing_Through_Simulation)

## Lincense

[BSD-3-Clause license](LICENSE)
