use std::{fmt::Debug, marker::PhantomData, ops::RangeBounds};

use crate::traits::Monoid;

/// A data structure that supports **range query point update** operations.
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

    // for debug
    query: PhantomData<Query>,
}
// ANCHOR_END: definition

impl<Query> SegmentTree<Query>
where
    Query: Monoid,
{
    /// Creates a new instance initialized with `n` [identity elements](crate::traits::Monoid::identity()).
    ///
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
        let data = Vec::from_iter(std::iter::repeat_with(<Query as Monoid>::identity).take(n << 1))
            .into_boxed_slice();

        Self {
            data,
            query: PhantomData,
        }
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
    pub fn len(&self) -> usize {
        self.data.len() >> 1
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
        self.data[self.data.len() >> 1..].iter()
    }

    #[inline]
    fn inner_index(&self, i: usize) -> usize {
        self.data.len() / 2 + i
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
            std::ops::Bound::Unbounded => self.data.len() / 2,
        };

        [self.inner_index(l), self.inner_index(r)]
    }

    /// Replaces i-th element with given `element`.
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

    /// Replaces i-th element using `f`.
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

    /// Answers query for the given range.
    ///
    /// If the given range is empty, returns [the identity element](crate::traits::Monoid::identity()).
    ///
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
        if l ^ r == 1 {
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

    /// Answers query for i-th element.
    ///
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
    pub fn point_query(&self, i: usize) -> &<Query as Monoid>::Set {
        let i = self.inner_index(i);
        &self.data[i]
    }
}

impl<Query> From<Vec<<Query as Monoid>::Set>> for SegmentTree<Query>
where
    Query: Monoid,
{
    fn from(values: Vec<<Query as Monoid>::Set>) -> Self {
        let mut data = Vec::from_iter(
            std::iter::repeat_with(<Query as Monoid>::identity)
                .take(values.len())
                .chain(values),
        )
        .into_boxed_slice();

        for i in (1..data.len() / 2).rev() {
            data[i] = <Query as Monoid>::combine(&data[i * 2], &data[i * 2 + 1])
        }

        Self {
            data,
            query: PhantomData,
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
                query: PhantomData,
            }
        } else {
            Self::from(Vec::from_iter(iter))
        }
    }
}

impl<Query> Debug for SegmentTree<Query>
where
    Query: Monoid,
    <Query as Monoid>::Set: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SegmentTree")
            .field("data", &self.data)
            .field("query", &self.query)
            .finish()
    }
}

impl<Query> Clone for SegmentTree<Query>
where
    Query: Monoid,
    <Query as Monoid>::Set: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            query: self.query,
        }
    }
}

#[cfg(test)]
mod test_range_query {
    use rand::Rng;

    use crate::{SegmentTree, ops::Add};

    fn template(n: usize) {
        let point_add_range_sum = SegmentTree::<Add<usize>>::from_iter(0..n);
        for i in 0..n {
            for j in i + 1..n {
                assert_eq!(
                    point_add_range_sum.range_query(i..j),
                    (i + j - 1) * (j - i) / 2
                )
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
