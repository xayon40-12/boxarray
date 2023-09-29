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
    mem::transmute,
};

mod private {
    use std::marker::PhantomData;

    /// Type-level list of const generic usize.
    pub trait CUList {
        type CoordType;
    }
    type CoordType<A> = <A as CUList>::CoordType;

    /// Value constructor for `CUList`. Represend a single value not in an array.
    pub struct Value {}
    impl CUList for Value {
        type CoordType = ();
    }

    /// Array constructor for `CUList`. Represent the outter-most array that contains the other nested arrays and its own size.
    pub struct Array<L: CUList, const N: usize> {
        _l: PhantomData<L>,
    }
    impl<L: CUList, const N: usize> CUList for Array<L, N> {
        type CoordType = (usize, L::CoordType);
    }

    /// Convert the impl type to a value of type `T`.
    pub trait Reify<T> {
        fn reify() -> T;
    }

    impl Reify<usize> for Value {
        fn reify() -> usize {
            0
        }
    }
    impl<L: CUList + Reify<usize>, const N: usize> Reify<usize> for Array<L, N> {
        fn reify() -> usize {
            1 + L::reify()
        }
    }

    impl Reify<CoordType<Value>> for Value {
        fn reify() -> CoordType<Value> {
            ()
        }
    }
    impl<L: CUList + Reify<CoordType<L>>, const N: usize> Reify<CoordType<Array<L, N>>>
        for Array<L, N>
    {
        fn reify() -> CoordType<Array<L, N>> {
            (N, L::reify())
        }
    }

    /// Constrains valid nested arrays.
    pub trait Arrays<E, L: CUList> {}
    impl<E> Arrays<E, Value> for E {}
    impl<E, L: CUList, A: Arrays<E, L>, const N: usize> Arrays<E, Array<L, N>> for [A; N] {}
}
use private::*;

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
pub fn boxarray<E: Copy, L: CUList, A: Arrays<E, L>>(e: E) -> Box<A> {
    unsafe {
        let ptr = alloc_zeroed(Layout::new::<A>());
        let st = std::mem::size_of::<A>();
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
