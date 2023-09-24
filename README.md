# boxarray.rs  
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/xayon40-12/boxarray/rust.yml?label=build/tests&logo=github)  

Safe way to allocate and initialize nested arrays directly on the heap in Rust.  

## Usage

In order to initialize a Boxed nested-array, simply call the `boxarray` function and give it the value (here `v`) to initialize with:
```rust
  let v = 7.0;
  let a: Box<[[[f64; 10]; 2]; 4]> = boxarray::boxarray(v);
```
