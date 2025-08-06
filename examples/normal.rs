// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_range_sum

use proconio::{fastout, input};
use seg_lib::{normal::SegmentTree, provider::AddProvider};

#[fastout]
fn main() {
    input! { n: usize, q: usize, a: [u64; n], }

    let mut seg_tree = SegmentTree::<_, _, AddProvider, AddProvider>::from(a);

    for _ in 0..q {
        input! { flag: u8, }

        if flag == 0 {
            input! { p: usize, x: u64, }

            seg_tree.point_update(p, x);
        } else if flag == 1 {
            input! { l: usize, r: usize, }

            println!("{}", seg_tree.range_query(l..r))
        } else {
            unreachable!()
        }
    }
}
