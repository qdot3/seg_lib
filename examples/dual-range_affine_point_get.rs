// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_point_get

use proconio::{fastout, input};
use seg_lib::{dual::DualSegmentTree, provider::Affine};

#[fastout]
fn main() {
    input! { n: usize, q: usize, a: [u64; n], }

    let mut dst = DualSegmentTree::<Affine<u64>>::new(n);

    for _ in 0..q {
        input! { flag: u8, }

        if flag == 0 {
            input! { l: usize, r: usize, b: u64, c: u64, }

            dst.range_update(l..r, &[b, c]);
        } else if flag == 1 {
            input! { i: usize, }

            let [tilt, offset] = dst.point_query(i);
            println!("{}", tilt * a[i] + offset);
        } else {
            unreachable!()
        }

        #[cfg(debug_assertions)]
        eprintln!("{:?}", dst)
    }
}
