use std::{
    alloc::{alloc_zeroed, Layout},
    marker::PhantomData,
    mem::transmute,
};

pub trait Nat {}
pub struct Z {}
impl Nat for Z {}
pub struct S<N: Nat> {
    _s: PhantomData<N>,
}
impl<N: Nat> Nat for S<N> {}
type N1 = S<Z>;
type N2 = S<N1>;
type N3 = S<N2>;
type N4 = S<N3>;
type N5 = S<N4>;
type N6 = S<N5>;
type N7 = S<N6>;
type N8 = S<N7>;
type N9 = S<N8>;
type N10 = S<N9>;

pub trait Fin<N: Nat> {}
pub struct FZ {}
impl<N: Nat> Fin<N> for FZ {}
pub struct FS<N: Nat, F: Fin<N>> {
    _n: PhantomData<N>,
    _f: PhantomData<F>,
}
impl<N: Nat, F: Fin<N>> Fin<S<N>> for FS<N, F> {}

type MaxSize = N10;
pub trait Arrays<E, FN: Fin<MaxSize>> {}
impl<E: Copy, const N: usize> Arrays<E, FZ> for [E; N] {}
impl<E: Copy, N: Nat, FN: Fin<MaxSize> + Fin<N>, A: Arrays<E, FN>, const S: usize>
    Arrays<E, FS<N, FN>> for [A; S]
where
    FS<N, FN>: Fin<MaxSize>,
{
}

pub fn boxarray<FN: Fin<MaxSize>, T: Arrays<E, FN>, E: Copy>(e: E) -> Box<T> {
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
