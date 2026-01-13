//! Safe way to allocate and initialize nested arrays directly on the heap inside a `Box`.
//!
//! ## Usage
//!
//! In order to initialize a Boxed nested-array, simply call the `boxarray` function and give it the value (here `v`) to initialize with:
//! ```
//!   let v = 7.0;
//!   let a: Box<[[[f64; 3]; 2]; 4]> = boxarray::boxarray(v);
//! ```
//!
//! The initialization can also be done with a function that takes the coordinates in nested tuples as arguments by using `boxarray_` instead:
//! ```
//!   let f = |((((), i), j), k)| (i+j*k) as usize;
//!   let a: Box<[[[usize; 3]; 2]; 4]> = boxarray::boxarray_(f);
//! ```
use std::{
    alloc::{Layout, alloc},
    mem::transmute,
};

mod private {
    use std::marker::PhantomData;

    /// Type-level list of const generic usize.
    pub trait CUList {
        type CoordType;
    }
    /// Type operator that return the CoordType of o CUList, which is a type representing nested tuple of usize, where the number of nesting is the same as the number of array nesting the CUList represent.
    pub type CoordType<A> = <A as CUList>::CoordType;

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
        type CoordType = (L::CoordType, usize);
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
            (L::reify(), N)
        }
    }

    /// Product for recursive types
    pub trait Product<T> {
        fn product() -> T;
    }
    impl Product<usize> for Value {
        fn product() -> usize {
            1
        }
    }
    impl<L: CUList + Product<usize>, const N: usize> Product<usize> for Array<L, N> {
        fn product() -> usize {
            N * L::product()
        }
    }

    pub trait IndexCoord<L: CUList> {
        fn coords(i: usize) -> CoordType<L>;
    }
    impl IndexCoord<Value> for Value {
        fn coords(_: usize) -> CoordType<Value> {
            ()
        }
    }
    impl<L: CUList + IndexCoord<L> + Product<usize>, const N: usize> IndexCoord<Array<L, N>>
        for Array<L, N>
    {
        fn coords(i: usize) -> CoordType<Array<L, N>> {
            let prod = L::product();

            (L::coords(i % prod), i / prod)
        }
    }

    /// Constrains valid nested arrays.
    pub trait Arrays<E, L: CUList> {}
    impl<E> Arrays<E, Value> for E {}
    impl<E, L: CUList, A: Arrays<E, L>, const N: usize> Arrays<E, Array<L, N>> for [A; N] {}
}
use private::*;
pub use private::{Array, Value};

/// The `boxarray` function allow to allocate nested arrays directly on the heap inside a `Box` and initialize it with a constant value of type `E`.
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
/// Zero sized type.
/// ```
/// fn zero_sized_type() {
///     #[derive(Clone, Copy, Debug, PartialEq, Eq)]
///     struct ZST;
///     let a: Box<[[[ZST; 10]; 2]; 4]> = boxarray::boxarray(ZST);
///     assert_eq!(*a, [[[ZST; 10]; 2]; 4]);
/// }
/// ```
///
/// If the type of the value to initialize with does not correspond, a compiler will be raised.
/// ```compile_fail
/// fn nested_array_wrong_type() {
///     let a: Box<[[[f64; 10]; 2]; 4]> = boxarray::boxarray(7.0f32);
/// }
/// ```
///
/// If the type to initialize is not only composed of nested arrays, a compiler will be raised.
/// ```compile_fail
/// fn nested_array_wrong_type() {
///     let a: Box<[[([f64; 10], [f64; 10]); 2]; 4]> = boxarray::boxarray(7.0);
/// }
/// ```
///
pub fn boxarray<E: Clone, L: CUList, A: Arrays<E, L>>(e: E) -> Box<A> {
    unsafe {
        let ptr = alloc(Layout::new::<A>());
        let se = std::mem::size_of::<E>();
        if se != 0 {
            let st = std::mem::size_of::<A>();
            let n = st / se;
            let arr: *mut E = transmute(ptr);
            for i in 0..n {
                std::ptr::write(arr.add(i), e.clone());
            }
        }
        Box::from_raw(std::mem::transmute(ptr))
    }
}

/// Same as `boxarray` but use a fonction that takes nested tuples of `usize` as coordinates and return a value of type `E` to initialize every cells.
///
/// # Examples
///
/// Zero-size array (i.e. a simple value)
/// ```
/// fn signle_array() {
///     let a: Box<u32> = boxarray::boxarray_(|()| 1);
///     assert_eq!(*a, 1u32);
/// }
/// ```
///
/// Single array.
/// ```
/// fn signle_array() {
///     let a: Box<[u32; 4]> = boxarray::boxarray_(|((),i)| i as  u32);
///     assert_eq!(*a, [0,1,2,3]);
/// }
/// ```
///
/// Nested array.
/// ```
/// fn nested_array() {
///     let a: Box<[[[i32; 3]; 2]; 4]> = boxarray::boxarray_(|((((),i),j),k)| (i+j*k) as i32);
///     let mut sol = [[[0i32; 3]; 2]; 4];
///     for k in 0..4 {
///         for j in 0..2 {
///             for i in 0..3 {
///                 sol[k][j][i] = (i+j*k) as i32;
///             }
///         }
///     }
///     assert_eq!(*a, sol);
/// }
/// ```
///
/// Fails to compile when the number of coordinates are not the same as the dimension of the nested arrays.
/// ```compile_fail
/// fn nested_array() {
///     let a: Box<[[[i32; 3]; 2]; 4]> = boxarray::boxarray_(|(((),i),j)| i as i32);
/// }
/// ```
/// ```compile_fail
/// fn nested_array() {
///     let a: Box<[[[i32; 3]; 2]; 4]> = boxarray::boxarray_(|(((((),i),j),k),l)| i as i32);
/// }
/// ```
///
pub fn boxarray_<E, L: CUList + IndexCoord<L>, A: Arrays<E, L>, F: Fn(CoordType<L>) -> E>(
    f: F,
) -> Box<A> {
    unsafe {
        let ptr = alloc(Layout::new::<A>());
        let se = std::mem::size_of::<E>();
        if se != 0 {
            let st = std::mem::size_of::<A>();
            let n = st / se;
            let arr: *mut E = transmute(ptr);
            for i in 0..n {
                std::ptr::write(arr.add(i), f(L::coords(i)));
            }
        }
        Box::from_raw(std::mem::transmute(ptr))
    }
}
