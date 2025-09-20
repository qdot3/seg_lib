use std::{
    fmt::Debug,
    ops::{Range, RangeBounds},
};

use num_traits::WrappingShl;

use crate::{traits::Monoid, utility::convert_range};

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

    /// Calculates all buffer segments in bottom-to-top order.
    ///
    /// # Time complexity
    ///
    /// *Θ*(*N*)
    #[inline]
    fn build(&mut self) -> &mut Self {
        for i in (1..self.len_or_offset).rev() {
            self.data[i] = <Query as Monoid>::combine(&self.data[i * 2], &self.data[i * 2 + 1])
        }
        self
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

    #[doc = include_str!("../doc/point_update.md")]
    ///
    /// # Time complexity
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
        R: RangeBounds<usize> + Debug,
    {
        let range = convert_range(range, 0..self.len_or_offset);
        if range.is_empty() {
            return <Query as Monoid>::identity();
        }

        let [mut l, mut r] = {
            // Consumes range and avoids copy
            let Range { start, end } = range;
            let [l, r] = [self.inner_index(start), self.inner_index(end)];
            [l >> l.trailing_zeros(), r >> r.trailing_zeros()]
        };
        let (mut acc_l, mut acc_r) = (<Query as Monoid>::identity(), <Query as Monoid>::identity());
        while {
            // This is branchy but necessary for avoiding invalid buffers. ...really?
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
    #[inline]
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
        assert!(start <= self.len_or_offset);

        let mut i = self.inner_index(start);
        let mut segment_size = 1.wrapping_shl(i.trailing_zeros());
        i = i.wrapping_shr(i.trailing_zeros());
        let mut combined = <Query as Monoid>::identity();

        let mut tmp;
        // The first condition ensures next segment is valid.
        while start + segment_size <= self.len_or_offset && {
            tmp = <Query as Monoid>::combine(&combined, &self.data[i]);
            pred(&tmp)
        } {
            combined = tmp;
            start += segment_size;
            i += 1;

            segment_size <<= i.trailing_zeros();
            i >>= i.trailing_zeros();
        }

        if start == self.len_or_offset {
            return self.len_or_offset;
        }

        (i, segment_size) = {
            i = self.inner_index(start);
            // never panic since `self.len_or_offset - start > 0`.
            let shift = (self.len_or_offset - start).ilog2().min(i.trailing_zeros());
            (i >> shift, 1 << shift)
        };
        while {
            tmp = <Query as Monoid>::combine(&combined, &self.data[i]);

            // branchless if block
            {
                // Checks whether the segment is valid.
                let is_ok = start + segment_size <= self.len_or_offset && pred(&tmp);
                combined = if is_ok { tmp } else { combined };
                i += if is_ok { 1 } else { 0 };
                start += if is_ok { segment_size } else { 0 };
            }

            i <<= 1;
            segment_size >>= 1;

            i < self.len_or_offset * 2
        } {}

        start
    }

    /// Returns the largest index `start` such that:
    ///
    /// ```text
    /// pred(self.range_query(i..end)) == true   for ∀ i ∈ [start, end]
    /// pred(self.range_query(i..end)) == false  for ∀ i ∈ [0, start - 1]
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
    pub fn partition_start<P>(&self, mut end: usize, pred: P) -> usize
    where
        P: Fn(&<Query as Monoid>::Set) -> bool,
    {
        // See `partition_end()` for details.

        assert!(end <= self.len_or_offset);

        let mut i = self.inner_index(end);
        let mut segment_size = 1.wrapping_shl(i.trailing_zeros());
        i = i.wrapping_shr(i.trailing_zeros());
        let mut combined = <Query as Monoid>::identity();

        let mut tmp;
        while end >= segment_size && {
            // i > 0 && i % 2 == 1
            tmp = <Query as Monoid>::combine(&combined, &self.data[i - 1]);
            pred(&tmp)
        } {
            combined = tmp;
            end -= segment_size;
            i -= 1;

            segment_size <<= i.trailing_zeros();
            i >>= i.trailing_zeros();
        }

        if end == 0 {
            return 0;
        }

        (i, segment_size) = {
            i = self.inner_index(end);
            let shift = end.ilog2().min(i.trailing_zeros());
            (i >> shift, 1 << shift)
        };
        while {
            tmp = <Query as Monoid>::combine(&combined, &self.data[i - 1]);

            // branchless if block
            {
                let is_ok = pred(&tmp) && end >= segment_size;
                combined = if is_ok { tmp } else { combined };
                i -= if is_ok { 1 } else { 0 };
                end -= if is_ok { segment_size } else { 0 };
            }

            i <<= 1;
            segment_size >>= 1;

            i <= self.len_or_offset * 2
        } {}

        end
    }
}

impl<Query> From<Vec<<Query as Monoid>::Set>> for SegmentTree<Query>
where
    Query: Monoid,
{
    fn from(values: Vec<<Query as Monoid>::Set>) -> Self {
        let n = values.len();
        let data = Vec::from_iter(
            std::iter::repeat_with(<Query as Monoid>::identity)
                .take(n)
                .chain(values),
        )
        .into_boxed_slice();

        let mut tree = Self {
            data,
            len_or_offset: n,
        };
        tree.build();

        tree
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
            let data = Vec::from_iter(
                std::iter::repeat_with(<Query as Monoid>::identity)
                    .take(min)
                    .chain(iter),
            )
            .into_boxed_slice();

            let mut tree = Self {
                data,
                len_or_offset: min,
            };
            tree.build();

            tree
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
mod partition_end {
    use rand::Rng;

    use crate::{SegmentTree, ops::Add};

    /// ANCHOR: test
    #[test]
    fn ones() {
        const MAX_SIZE: isize = 200;
        const OFFSET: isize = 10;

        for size in 0..MAX_SIZE {
            let range_sum_query =
                SegmentTree::<Add<isize>>::from_iter(std::iter::repeat_n(1, size as usize));
            for start in 0..=size {
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
    /// ANCHOR_END: test

    #[test]
    fn zero_one() {
        const SIZE: u32 = 100;

        // *O*(*N*)
        fn naive(values: &Vec<u32>, start: usize, pred: impl Fn(&u32) -> bool) -> usize {
            let additional = values[start..]
                .iter()
                .scan(0, |acc, v| {
                    *acc += v;
                    Some(*acc)
                })
                .take_while(|v| pred(v))
                .count();
            start + additional
        }

        let mut rng = rand::rng();
        for size in 0..=SIZE {
            let values = Vec::from_iter(
                std::iter::repeat_with(|| rng.random_range(0..=1)).take(size as usize),
            );
            let range_sum_query = SegmentTree::<Add<_>>::from(values.clone());

            for start in 0..=size as usize {
                for sum in 0..=size {
                    assert_eq!(
                        range_sum_query.partition_end(start, |v| *v < sum),
                        naive(&values, start, |v| *v < sum)
                    )
                }
            }
        }
    }
}

#[cfg(test)]
mod partition_start {
    use rand::Rng;

    use crate::{SegmentTree, ops::Add};

    #[test]
    fn ones() {
        const MAX_SIZE: isize = 200;
        const OFFSET: isize = 10;

        for size in 0..MAX_SIZE {
            let range_sum_query =
                SegmentTree::<Add<isize>>::from_iter(std::iter::repeat_n(1, size as usize));
            for end in 0..=size {
                for sum in -OFFSET..=size as isize + OFFSET {
                    assert_eq!(
                        range_sum_query.partition_start(end as usize, |&v| v <= sum),
                        (end - sum).clamp(0, end) as usize,
                        "size: {size}, end: {end}, sum: {sum}"
                    )
                }
            }
        }
    }

    #[test]
    fn zero_one() {
        const SIZE: u32 = 100;

        // *O*(*N*)
        fn naive(values: &Vec<u32>, end: usize, pred: impl Fn(&u32) -> bool) -> usize {
            let additional = values[..end]
                .iter()
                .rev()
                .scan(0, |acc, v| {
                    *acc += v;
                    Some(*acc)
                })
                .take_while(|v| pred(v))
                .count();
            end - additional
        }

        let mut rng = rand::rng();
        for size in 0..=SIZE {
            let values = Vec::from_iter(
                std::iter::repeat_with(|| rng.random_range(0..=1)).take(size as usize),
            );
            let range_sum_query = SegmentTree::<Add<_>>::from(values.clone());

            for end in 0..=size as usize {
                for sum in 0..=size {
                    assert_eq!(
                        range_sum_query.partition_start(end, |v| *v <= sum),
                        naive(&values, end, |v| *v <= sum),
                        "size: {size}, end: {end}, sum: {sum}, vex: {values:?}"
                    )
                }
            }
        }
    }
}

// pub struct IterMut<'a, Query>
// where
//     Query: Monoid,
// {
//     buffer: &'a mut [<Query as Monoid>::Set],
//     iter_mut: std::slice::IterMut<'a, <Query as Monoid>::Set>,
// }

// impl<'a, Query> IterMut<'a, Query>
// where
//     Query: Monoid,
// {
//     fn new(tree: &'a mut SegmentTree<Query>) -> Self {
//         let (buffer, data) = tree.data.split_at_mut(tree.len_or_offset);
//         Self {
//             buffer,
//             iter_mut: data.iter_mut(),
//         }
//     }
// }

// impl<'a, Query> Drop for IterMut<'a, Query>
// where
//     Query: Monoid,
// {
//     fn drop(&mut self) {
//         todo!("recalculate buffer")
//     }
// }

// impl<'a, Query> Deref for IterMut<'a, Query>
// where
//     Query: Monoid,
// {
//     type Target = std::slice::IterMut<'a, <Query as Monoid>::Set>;

//     fn deref(&self) -> &Self::Target {
//         &self.iter_mut
//     }
// }

// impl<'a, Query> DerefMut for IterMut<'a, Query>
// where
//     Query: Monoid,
// {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.iter_mut
//     }
// }
