// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_composite

use proconio::{fastout, input};
use seg_lib::{Monoid, SegmentTree};

const MOD: u64 = 998_244_353;

#[fastout]
fn main() {
    input! { n: usize, q: usize, ab: [(u64, u64); n], }

    let mut seg_tree = SegmentTree::<ModAffine<MOD>>::from(ab);

    for _ in 0..q {
        input! { flag: u8, }

        if flag == 0 {
            input! { p: usize, c: u64, d: u64, }

            seg_tree.point_update(p, (c, d));
        } else if flag == 1 {
            input! { l: usize, r: usize, x: u64, }

            let (a, b) = seg_tree.range_query(l..r);
            println!("{}", (a * x + b) % MOD)
        } else {
            unreachable!()
        }
    }
}

struct ModAffine<const MOD: u64>;

impl<const MOD: u64> Monoid for ModAffine<MOD> {
    type Set = (u64, u64);

    const IS_COMMUTATIVE: bool = false;

    fn identity() -> Self::Set {
        (1, 0)
    }

    fn combine(prev: &Self::Set, new: &Self::Set) -> Self::Set {
        (new.0 * prev.0 % MOD, (new.0 * prev.1 + new.1) % MOD)
    }
}
