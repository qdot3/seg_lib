use std::{fmt::Debug, marker::PhantomData, ops::RangeBounds};

use crate::traits::Monoid;

/// A data structure that supports *range query point update* operations.
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
    query: PhantomData<Query>,
}

impl<Query> SegmentTree<Query>
where
    Query: Monoid,
{
    /// Creates an instance initialized with `n` [`Monoid::identity`]s.
    ///
    /// # Example
    ///
    /// ```
    /// use seg_lib::{SegmentTree, provider::Add};
    ///
    /// let mut point_update_range_sum = SegmentTree::<Add<i32>>::new(10_000);
    /// ```
    ///
    /// # Time complexity
    ///
    /// *O*(*N*)
    pub fn new(n: usize) -> Self {
        let data = Vec::from_iter(std::iter::repeat_with(<Query as Monoid>::identity).take(n << 1))
            .into_boxed_slice();

        Self {
            data,
            query: PhantomData,
        }
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

    /// Replacing i-th data with `update`.
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    pub fn point_update(&mut self, i: usize, element: <Query as Monoid>::Set) {
        let mut i = self.inner_index(i);
        self.data[i] = element;
        while i > 1 {
            i >>= 1;
            self.data[i] = <Query as Monoid>::combine(&self.data[i << 1], &self.data[(i << 1) + 1])
        }
    }

    /// Updates i-th element using `f`.
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
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

    /// Returns a shared reference of i-th data.
    ///
    /// # Time complexity
    ///
    /// *O*(1)
    pub fn point_query(&self, i: usize) -> &<Query as Monoid>::Set {
        let i = self.inner_index(i);
        &self.data[i]
    }

    /// Answers the query for the given range.
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
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
