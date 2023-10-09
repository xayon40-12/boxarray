# boxarray
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/xayon40-12/boxarray/rust.yml?label=build/tests&logo=github)](https://github.com/xayon40-12/boxarray/actions)
[![Crates.io](https://img.shields.io/crates/v/boxarray.svg)](https://crates.io/crates/boxarray)
[![Docs.rs](https://docs.rs/boxarray/badge.svg)](https://docs.rs/boxarray)

Safe way to allocate and initialize nested arrays directly on the heap in Rust.  

## Usage

To use `boxarray` in your Rust project, simply add it as a dependency in your `Cargo.toml`:
```toml
[dependencies]
boxarray = "1.2.1"
```

Then import and use it in your project:
```rust
use boxarray::boxarray;
use boxarray::boxarray_;

fn main() {
  let v = 7.0;
  let a: Box<[[[f64; 3]; 2]; 4]> = boxarray(v);
  println!("{a:?}");

  let f = |((((), i), j), k)| (i+j*k) as usize;
  let a: Box<[[[usize; 3]; 2]; 4]> = boxarray_(f);
  println!("{a:?}");
}
```