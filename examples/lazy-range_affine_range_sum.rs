// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_range_sum

use proconio::{fastout, input};
use seg_lib::{LazySegmentTree, provider::Add, traits::MonoidWithAction};

const MOD: u64 = 998_244_353;

#[fastout]
fn main() {
    input! { n: usize, q: usize, a: [u64; n], }

    let mut lst = LazySegmentTree::<Add<u64>, ModAffine<MOD>>::from(a);

    for _ in 0..q {
        input! { flag: u8, }

        if flag == 0 {
            input! { l: usize, r: usize, b: u64, c: u64, }

            lst.range_update(l..r, [b, c]);
        } else if flag == 1 {
            input! { l: usize, r: usize, }

            println!("{}", lst.range_query(l..r) % MOD);
        } else {
            unreachable!()
        }

        #[cfg(debug_assertions)]
        eprintln!("{:?}", lst)
    }
}

struct ModAffine<const MOD: u64>;

impl<const MOD: u64> MonoidWithAction for ModAffine<MOD> {
    type Map = [u64; 2];
    type Set = u64;

    const IS_COMMUTATIVE: bool = false;
    const USE_SEGMENT_SIZE: bool = true;

    fn identity() -> Self::Map {
        [1, 0]
    }

    fn combine(prev: &mut Self::Map, new: &Self::Map) {
        *prev = [new[0] * prev[0] % MOD, (new[0] * prev[1] + new[1]) % MOD]
    }

    fn act(mapping: &Self::Map, element: &Self::Set, size: Option<usize>) -> Self::Set {
        (mapping[0] * *element + size.unwrap() as u64 * mapping[1]) % MOD
    }
}
