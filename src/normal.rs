use std::{fmt::Debug, ops::RangeBounds};

use crate::traits::Monoid;

/// A data structure that supports **range query point update** operations.
///
/// # Example
///
/// ```
#[doc = include_str!("../examples/ex_segment_tree.rs")]
/// ```
// ANCHOR: definition
pub struct SegmentTree<Query>
where
    Query: Monoid,
{
    /// Use `Box<T>` because the length is significant as follows.
    ///
    /// - data\[0\]    : dummy node (meaningless)
    /// - data\[1..n\] : nodes to store the combined value of the children.
    /// - data\[n..2n\]: nodes to store value for each cell.
    data: Box<[<Query as Monoid>::Set]>,

    /// `len` (number of elements) and offset (dummy + cache)
    len_or_offset: usize,
    // /// This and its ancestors are all invalid (cached result may be broken)
    // min_invalid_node: usize,
}
// ANCHOR_END: definition

impl<Query> SegmentTree<Query>
where
    Query: Monoid,
{
    #[doc = include_str!("../doc/new.md")]
    /// # Example
    ///
    /// ```
    /// use seg_lib::{SegmentTree, ops::Add};
    ///
    /// let mut st = SegmentTree::<Add<i32>>::new(10_000);
    /// ```
    ///
    /// # Time complexity
    ///
    /// *O*(*N*)
    #[inline]
    pub fn new(n: usize) -> Self {
        Self::from_iter(std::iter::repeat_with(<Query as Monoid>::identity).take(n))
    }

    /// Returns the number of elements.
    ///
    /// # Time complexity
    ///
    /// *O*(1)
    ///
    /// # Example
    ///
    /// ```
    /// use seg_lib::{SegmentTree, ops::BitAnd};
    ///
    /// let n = 100;
    /// let st = SegmentTree::<BitAnd<u32>>::from_iter(0..n);
    ///
    /// assert_eq!(st.len(), n as usize)
    /// ```
    #[allow(clippy::len_without_is_empty)]
    #[inline]
    pub const fn len(&self) -> usize {
        self.len_or_offset
    }

    /// Returns an iterator over the elements
    ///
    /// # Time complexity
    ///
    /// *O*(*N*)
    ///
    /// # Example
    ///
    /// ```
    /// use seg_lib::{SegmentTree, ops::BitOr};
    ///
    /// let n = 100;
    /// let st = SegmentTree::<BitOr<u32>>::from_iter(0..n);
    ///
    /// assert_eq!(
    ///     Vec::from_iter(st.iter().copied()),
    ///     Vec::from_iter(0..n),
    /// )
    /// ```
    #[inline]
    #[must_use = "iterators are lazy and do nothing unless consumed"]
    pub fn iter(&self) -> std::slice::Iter<'_, <Query as Monoid>::Set> {
        self.data[self.len_or_offset..].iter()
    }

    #[inline]
    const fn inner_index(&self, i: usize) -> usize {
        self.len_or_offset + i
    }

    /// Returns `[l, r)` on `self.data`.
    #[inline]
    fn inner_range<R>(&self, range: R) -> [usize; 2]
    where
        R: RangeBounds<usize>,
    {
        let l = match range.start_bound() {
            std::ops::Bound::Included(l) => *l,
            std::ops::Bound::Excluded(l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            std::ops::Bound::Included(r) => r + 1,
            std::ops::Bound::Excluded(r) => *r,
            std::ops::Bound::Unbounded => self.len_or_offset,
        };

        [self.inner_index(l), self.inner_index(r)]
    }

    #[doc = include_str!("../doc/point_update.md")]
    ///
    ///  # Time complexity
    ///
    /// *O*(log *N*)
    ///
    /// # Example
    ///
    /// ```
    /// use seg_lib::{SegmentTree, ops::Mul};
    ///
    /// let mut st = SegmentTree::<Mul<i32>>::new(100);
    /// assert_eq!(st.range_query(..), 1);
    ///
    /// st.point_update(50, 2);
    /// assert_eq!(st.point_query(50), &2);
    /// assert_eq!(st.range_query(..50), 1);
    /// ```
    pub fn point_update(&mut self, i: usize, element: <Query as Monoid>::Set) {
        let mut i = self.inner_index(i);
        self.data[i] = element;
        while i > 1 {
            i >>= 1;
            self.data[i] = <Query as Monoid>::combine(&self.data[i << 1], &self.data[(i << 1) + 1])
        }
    }

    /// Updates the `i`-th element by applying the function `f`.
    ///
    /// # Panics
    ///
    /// Panics if `i` is out of bounds.
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    ///
    /// # Example
    ///
    /// ```
    /// use seg_lib::{SegmentTree, ops::Max};
    ///
    /// let mut st = SegmentTree::<Max<i32>>::new(100);
    /// // initialized with `None`
    /// assert_eq!(st.range_query(..), None);
    ///
    /// st.point_update(50, Some(2));
    /// st.point_update(60, Some(10));
    /// assert_eq!(st.range_query(..), Some(10));
    ///
    /// // add 100 to 50th element
    /// st.point_update_with(50, |element| element.map(|v| v + 100));
    /// assert_eq!(st.range_query(..), Some(102));
    /// ```
    pub fn point_update_with<F>(&mut self, i: usize, f: F)
    where
        F: FnOnce(&<Query as Monoid>::Set) -> <Query as Monoid>::Set,
    {
        let mut i = self.inner_index(i);
        self.data[i] = f(&self.data[i]);
        while i > 1 {
            i >>= 1;
            self.data[i] = <Query as Monoid>::combine(&self.data[i << 1], &self.data[(i << 1) + 1])
        }
    }

    #[doc = include_str!("../doc/range_query.md")]
    /// # Time complexity
    ///
    /// *O*(log *N*)
    ///
    /// # Example
    ///
    /// ```
    /// use seg_lib::{SegmentTree, ops::LCM};
    ///
    /// let mut st = SegmentTree::<LCM<i32>>::new(100);
    /// assert_eq!(st.range_query(..), 1);
    ///
    /// st.point_update(30, 2);
    /// st.point_update(40, 5);
    /// st.point_update(50, 8);
    /// st.point_update(60, 7);
    /// assert_eq!(st.range_query(..=50), 5 * 8)
    /// ```
    pub fn range_query<R>(&self, range: R) -> <Query as Monoid>::Set
    where
        R: RangeBounds<usize>,
    {
        let [l, r] = self.inner_range(range);
        if l >= r {
            return <Query as Monoid>::identity();
        }
        // l + 1 == r because l < r except when overflow occurs
        if l + 1 == r {
            return <Query as Monoid>::combine(&<Query as Monoid>::identity(), &self.data[l]);
        }

        let [mut l, mut r] = [l >> l.trailing_zeros(), r >> r.trailing_zeros()];
        let (mut acc_l, mut acc_r) = (<Query as Monoid>::identity(), <Query as Monoid>::identity());
        while {
            if l >= r {
                acc_l = <Query as Monoid>::combine(&acc_l, &self.data[l]);
                l += 1;
                l >>= l.trailing_zeros()
            } else {
                r -= 1; // r > l >= 0
                acc_r = <Query as Monoid>::combine(&self.data[r], &acc_r);
                r >>= r.trailing_zeros();
            }

            l != r
        } {}

        <Query as Monoid>::combine(&acc_l, &acc_r)
    }

    #[doc = include_str!("../doc/point_query.md")]
    /// # Time complexity
    ///
    /// *O*(1)
    ///
    /// # Example
    ///
    /// ```
    /// use seg_lib::{SegmentTree, ops::BitXor};
    ///
    /// let mut st = SegmentTree::<BitXor<u32>>::new(100);
    /// assert_eq!(st.point_query(10), &0);
    ///
    /// st.point_update(10, 6);
    /// assert_eq!(st.point_query(10), &6);
    /// ```
    pub const fn point_query(&self, i: usize) -> &<Query as Monoid>::Set {
        let i = self.inner_index(i);
        &self.data[i]
    }

    /// Returns the largest index `end` such that:
    ///
    /// ```text
    /// pred(self.range_query(start..i)) == true   for ∀ i ∈ [start, end]
    /// pred(self.range_query(start..i)) == false  for ∀ i ∈ [end + 1, N]
    /// ```
    ///
    /// This is analogous to [`slice::partition_point`], but applied to
    /// range queries on a segment tree.
    ///
    /// # Constraints
    ///
    /// - `pred` must return `true` for the identity element.
    /// - Once `pred` returns `false` for some `i`, it must return `false`
    ///   for all larger `i`, that is the results must be partitioned.
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    ///
    /// # Example
    ///
    /// ```
    /// let st = seg_lib::SegmentTree::<seg_lib::ops::Add<i32>>::from(
    ///     vec![1, 1, 1, 0, 0, 0, 0, 1, 1, 1]
    /// );
    ///
    /// let start = 1;
    /// let sum = 2;
    /// let end = st.partition_end(start, |v| *v <= sum);
    /// assert_eq!(end, 7);
    /// assert!((start..end).all(|end| st.range_query(start..end) <= sum));
    /// assert!((end + 1..10).all(|end| st.range_query(start..end) > sum));
    /// ```
    pub fn partition_end<P>(&self, mut start: usize, pred: P) -> usize
    where
        P: Fn(&<Query as Monoid>::Set) -> bool,
    {
        assert!(start < self.len_or_offset);

        let mut i = self.inner_index(start);
        let mut segment_size = 1 << i.trailing_zeros();
        i >>= i.trailing_zeros();
        let mut combined = <Query as Monoid>::identity();

        while start + segment_size < self.len_or_offset {
            let tmp = <Query as Monoid>::combine(&combined, &self.data[i]);

            if pred(&tmp) {
                combined = tmp;

                start += segment_size;

                i += 1;
                segment_size <<= i.trailing_zeros();
                i >>= i.trailing_zeros();
            } else {
                break;
            }
        }

        if start == self.len_or_offset {
            return self.len_or_offset;
        }

        (i, segment_size) = {
            i = self.inner_index(start);
            let shift = (self.len_or_offset - start).ilog2().min(i.trailing_zeros());
            (i >> shift, 1 << shift)
        };
        while {
            let tmp = <Query as Monoid>::combine(&combined, &self.data[i]);

            if pred(&tmp) {
                combined = tmp;

                i += 1;
                start += segment_size;
            }

            i <<= 1;
            segment_size >>= 1;

            i < self.len_or_offset * 2
        } {}

        start.min(self.len_or_offset)
    }
}

impl<Query> From<Vec<<Query as Monoid>::Set>> for SegmentTree<Query>
where
    Query: Monoid,
{
    fn from(values: Vec<<Query as Monoid>::Set>) -> Self {
        let n = values.len();
        let mut data = Vec::from_iter(
            std::iter::repeat_with(<Query as Monoid>::identity)
                .take(n)
                .chain(values),
        )
        .into_boxed_slice();

        for i in (1..data.len() / 2).rev() {
            data[i] = <Query as Monoid>::combine(&data[i * 2], &data[i * 2 + 1])
        }

        Self {
            data,
            len_or_offset: n,
        }
    }
}

impl<Query> FromIterator<<Query as Monoid>::Set> for SegmentTree<Query>
where
    Query: Monoid,
{
    fn from_iter<I: IntoIterator<Item = <Query as Monoid>::Set>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let (min, max) = iter.size_hint();
        if Some(min) == max {
            let mut data = Vec::from_iter(
                std::iter::repeat_with(<Query as Monoid>::identity)
                    .take(min)
                    .chain(iter),
            )
            .into_boxed_slice();

            for i in (1..min).rev() {
                data[i] = <Query as Monoid>::combine(&data[i * 2], &data[i * 2 + 1])
            }

            Self {
                data,
                len_or_offset: min,
            }
        } else {
            Self::from(Vec::from_iter(iter))
        }
    }
}

impl<Query> Debug for SegmentTree<Query>
where
    Query: Monoid<Set: Debug>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SegmentTree")
            .field("data", &self.data)
            .field("len_or_offset", &self.len_or_offset)
            .finish()
    }
}

impl<Query> Clone for SegmentTree<Query>
where
    Query: Monoid<Set: Clone>,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            len_or_offset: self.len_or_offset,
        }
    }
}

#[cfg(test)]
mod range_query {
    use rand::Rng;

    use crate::{SegmentTree, ops::Add};

    fn template(n: usize) {
        let range_sum = SegmentTree::<Add<usize>>::from_iter(0..n);
        for i in 0..n {
            for j in i + 1..n {
                assert_eq!(range_sum.range_query(i..j), (i + j - 1) * (j - i) / 2)
            }
        }
    }

    #[test]
    fn pow2() {
        for i in 0..5 {
            template(1 << i);
        }
    }

    #[test]
    fn random() {
        let mut rng = rand::rng();
        for _ in 0..5 {
            template(rng.random_range(100..5_000));
        }
    }
}

#[cfg(test)]
mod partition_point {
    use std::ops::Range;

    use rand::Rng;

    use crate::{SegmentTree, ops::Add};

    /// *O*(*N*)
    fn naive_partition_end<P>(src: &[u32], start: usize, pred: P) -> usize
    where
        P: Fn(u32) -> bool,
    {
        let mut acc = 0;
        start
            + src[start..]
                .iter()
                .take_while(|&&v| {
                    acc += v;
                    pred(acc)
                })
                .count()
    }

    #[test]
    fn partition_end_ones() {
        const MAX_SIZE: isize = 200;
        const OFFSET: isize = 10;

        for size in 0..MAX_SIZE {
            let range_sum_query =
                SegmentTree::<Add<isize>>::from_iter(std::iter::repeat_n(1, size as usize));
            for start in 0..size {
                for sum in -OFFSET..=size as isize + OFFSET {
                    assert_eq!(
                        range_sum_query.partition_end(start as usize, |&v| v <= sum),
                        (start + sum).clamp(start, size) as usize,
                        "size: {size}, start: {start}, sum: {sum}"
                    )
                }
            }
        }
    }

    #[test]
    fn partition_end() {
        const SIZE: usize = 1_000;
        const RANGE: Range<u32> = 0..u32::MAX / SIZE as u32; // avoid overflow for range sum query

        let mut rng = rand::rng();

        let src = Vec::from_iter(std::iter::repeat_with(|| rng.random_range(RANGE)).take(SIZE));
        let range_sum_query = SegmentTree::<Add<_>>::from(src.clone());

        // for _ in 0..ITER_COUNT {
        let start = rng.random_range(0..SIZE);
        let sum = rng.random();
        println!("{start} {sum}");
        assert_eq!(
            range_sum_query.partition_end(start, |v| *v < sum),
            naive_partition_end(&src, start, |v| v < sum)
        );
        // }
    }
}
