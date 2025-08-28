// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_composite_large_array

/*
5 8
1 2 4 692819383
0 3 799637677 179913266
1 3 5 201246867
1 2 4 21564799
0 1 612278332 480645345
0 4 591363278 253325503
0 4 329123774 683907575
1 2 4 374077434
*/
/*
1000000000 8
1 120999146 136987924 40280999
1 234164454 805152546 129099933
1 410134225 983937488 668461222
1 394300887 885417013 86086437
1 57893312 221329455 584381871
1 728257729 738660729 551666498
0 165573287 512342481 452470821
1 340240729 965382636 953292241
*/
use proconio::{fastout, input};
use seg_lib::{DynamicSegmentTree, traits::Monoid};

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
