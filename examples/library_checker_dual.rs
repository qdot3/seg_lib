// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_point_get

use proconio::{fastout, input};
use seg_lib::{DualSegmentTree, Monoid};

const MOD: u64 = 998_244_353;

#[fastout]
fn main() {
    input! { n: usize, q: usize, a: [u64; n], }

    let mut dst = DualSegmentTree::<ModAffine<MOD>>::new(n);

    for _ in 0..q {
        input! { flag: u8, }

        if flag == 0 {
            input! { l: usize, r: usize, b: u64, c: u64, }

            dst.range_update(l..r, &[b, c]);
        } else if flag == 1 {
            input! { i: usize, }

            let [tilt, offset] = dst.point_query(i);
            println!("{}", (tilt * a[i] + offset) % MOD);
        } else {
            unreachable!()
        }
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
