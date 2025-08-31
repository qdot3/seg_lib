// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_range_sum

use proconio::{fastout, input};
use seg_lib::{LazySegmentTree, Monoid, MonoidAction};

const MOD: u64 = 998_244_353;

#[fastout]
fn main() {
    input! { n: usize, q: usize, a: [u64; n], }

    let mut lst = LazySegmentTree::<ModAdd<MOD>, ModAffine<MOD>>::from(a);

    for _ in 0..q {
        input! { flag: u8, }

        if flag == 0 {
            input! { l: usize, r: usize, b: u64, c: u64, }

            lst.range_update(l..r, &[b, c]);
        } else if flag == 1 {
            input! { l: usize, r: usize, }

            println!("{}", lst.range_query(l..r) % MOD);
        } else {
            unreachable!()
        }

        #[cfg(debug_assertions)]
        eprintln!("{lst:?}")
    }
}

struct ModAdd<const MOD: u64>;

impl<const MOD: u64> Monoid for ModAdd<MOD> {
    type Set = u64;

    const IS_COMMUTATIVE: bool = true;

    fn identity() -> Self::Set {
        0
    }

    fn combine(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set {
        (lhs + rhs) % MOD
    }
}
struct ModAffine<const MOD: u64>;

impl<const MOD: u64> Monoid for ModAffine<MOD> {
    type Set = [u64; 2];

    const IS_COMMUTATIVE: bool = false;

    fn identity() -> Self::Set {
        [1, 0]
    }

    fn combine(prev: &Self::Set, new: &Self::Set) -> Self::Set {
        [new[0] * prev[0] % MOD, (new[0] * prev[1] + new[1]) % MOD]
    }
}

impl<const MOD: u64> MonoidAction for ModAffine<MOD> {
    type Map = ModAffine<MOD>;
    type Set = ModAdd<MOD>;

    const USE_SEGMENT_SIZE: bool = true;

    fn act(
        mapping: &<Self::Map as Monoid>::Set,
        element: &mut <Self::Set as Monoid>::Set,
        size: Option<usize>,
    ) {
        *element = (mapping[0] * *element + size.unwrap() as u64 * mapping[1]) % MOD
    }
}
