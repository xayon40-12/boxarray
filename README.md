# boxarray.rs

Safe way to allocate and initialize nested arrays directly on the heap in Rust.

## Usage

In order to initialize a Boxed nested-array, simply call the `boxarray` function and give it the value (here `v`) to initialize with:
```rust
  let v = 7.0;
  let a: Box<[[[f64; 10]; 2]; 4]> = boxarray::boxarray(v);
```
