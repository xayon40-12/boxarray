//! Safe way to allocate and initialize nested arrays directly on the heap inside a `Box`.
//!
//! ## Usage
//!
//! In order to initialize a Boxed nested-array, simply call the `boxarray` function and give it the value (here `v`) to initialize with:
//! ```
//!   let v = 7.0;
//!   let a: Box<[[[f64; 3]; 2]; 4]> = boxarray::boxarray(v);
//! ```
use std::{
    alloc::{alloc_zeroed, Layout},
    marker::PhantomData,
    mem::transmute,
};

/// Type-level list of const generic usize.
pub trait CUList {}

/// Value constructor for `CUList`. Represend a single value not in an array.
pub struct Value {}
impl CUList for Value {}

/// Array constructor for `CUList`. Represent the outter-most array that contains the other nested arrays and its own size.
pub struct Array<L: CUList, const N: usize> {
    _l: PhantomData<L>,
}
impl<L: CUList, const N: usize> CUList for Array<L, N> {}

/// Constrains valid nested arrays.
pub trait Arrays<E, L: CUList> {}
impl<T: Copy> Arrays<T, Value> for T {}
impl<T: Copy, L: CUList, A: Arrays<T, L>, const N: usize> Arrays<T, Array<L, N>> for [A; N] {}

/// The `boxarray` function allow to allocate and initialize nested arrays directly on the heap inside a `Box`.
///
/// # Examples
///
/// Zero-size array (i.e. a simple value)
/// ```
/// fn signle_array() {
///     let a: Box<u32> = boxarray::boxarray(1);
///     assert_eq!(*a, 1u32);
/// }
/// ```
///
/// Single array.
/// ```
/// fn signle_array() {
///     let a: Box<[u32; 10]> = boxarray::boxarray(1);
///     assert_eq!(*a, [1u32; 10]);
/// }
/// ```
///
/// Nested array.
/// ```
/// fn nested_array() {
///     let a: Box<[[[f64; 10]; 2]; 4]> = boxarray::boxarray(7.0);
///     assert_eq!(*a, [[[7f64; 10]; 2]; 4]);
/// }
/// ```
///
/// If the type of the value to initialize with does not correspond, a compiler will be raised.
/// ```compile_fail
/// fn nested_array_wrong_type() {
///     let a: Box<[[[f64; 10]; 2]; 4]> = boxarray::boxarray(7.0f32);
///     assert_eq!(*a, [[[7f64; 10]; 2]; 4]);
/// }
/// ```
///
/// If the type to initialize is not only composed of nested arrays, a compiler will be raised.
/// ```compile_fail
/// fn nested_array_wrong_type() {
///     let a: Box<[[([f64; 10], [f64; 10]); 2]; 4]> = boxarray::boxarray(7.0);
///     assert_eq!(*a, [[[7f64; 10]; 2]; 4]);
/// }
/// ```
///
pub fn boxarray<E: Copy, L: CUList, T: Arrays<E, L>>(e: E) -> Box<T> {
    unsafe {
        let ptr = alloc_zeroed(Layout::new::<T>());
        let st = std::mem::size_of::<T>();
        let se = std::mem::size_of::<E>();
        assert!(st % se == 0);
        let n = st / se;
        let arr: *mut E = transmute(ptr);
        for i in 0..n {
            *arr.add(i) = e;
        }
        std::mem::transmute(ptr)
    }
}
