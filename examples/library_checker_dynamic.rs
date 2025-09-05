// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_composite_large_array

use proconio::{fastout, input};
use seg_lib::{DynamicSegmentTree, Monoid};

const MOD: u64 = 998_244_353;

#[fastout]
fn main() {
    input! { n: isize, q: usize, }

    let mut dst = DynamicSegmentTree::<ModAffine<MOD>>::with_capacity(0..n, q).unwrap();

    for _ in 0..q {
        input! { flag: u8, }

        if flag == 0 {
            input! { p: isize, c: u64, d: u64, }

            dst.point_update(p, [c, d])
        } else if flag == 1 {
            input! { l: isize, r: isize, x: u64, }

            let [a, b] = dst.range_query(l..r);
            println!("{}", (a * x + b) % MOD);
        } else {
            unreachable!()
        }

        #[cfg(debug_assertions)]
        eprintln!("{:?}", dst);
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
