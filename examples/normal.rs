// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_range_sum

use proconio::{fastout, input};
use seg_lib::{SegmentTree, provider::Add};

#[fastout]
fn main() {
    input! { n: usize, q: usize, a: [u64; n], }

    let mut seg_tree = SegmentTree::<Add<u64>>::from(a);

    for _ in 0..q {
        input! { flag: u8, }

        if flag == 0 {
            input! { p: usize, x: u64, }

            seg_tree.point_update(p, x + seg_tree.point_query(p));
        } else if flag == 1 {
            input! { l: usize, r: usize, }

            println!("{}", seg_tree.range_query(l..r))
        } else {
            unreachable!()
        }

        #[cfg(debug_assertions)]
        eprintln!("{:?}", seg_tree);
    }
}
