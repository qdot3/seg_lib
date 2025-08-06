use std::{
    marker::PhantomData,
    ops::{Range, RangeBounds},
};

use crate::traits::{Query, Update};

#[derive(Debug, Clone)]
pub struct SegmentTree<T, U, QueryProvider, UpdateProvider>
where
    T: Clone, // used in `range_query()` because it is NOT allowed to return a reference to a temporary value.
    QueryProvider: Query<T>,
    UpdateProvider: Update<U, Arg = T>,
{
    /// Use `Box<T>` because the length is significant as follows.
    ///
    /// - data\[0\]    : dummy node (meaningless)
    /// - data\[1..n\] : nodes to store the combined value of the children.
    /// - data\[n..2n\]: nodes to store value for each cell.
    data: Box<[T]>,
    update_type: PhantomData<U>,
    query_adapter: PhantomData<QueryProvider>,
    update_adapter: PhantomData<UpdateProvider>,
}

impl<T, U, QueryProvider, UpdateProvider> SegmentTree<T, U, QueryProvider, UpdateProvider>
where
    T: Clone,
    QueryProvider: Query<T>,
    UpdateProvider: Update<U, Arg = T>,
{
    /// Creates a [`SegmentTree`] initialized with `n` [`Query::identity`].
    ///
    /// # Example
    ///
    /// ```
    /// use seg_lib::{normal::SegmentTree, provider::AddProvider};
    ///
    /// let mut point_add_range_sum = SegmentTree::<u32, _, AddProvider, AddProvider>::new(10_000);
    /// ```
    ///
    /// # Time complexity
    ///
    /// *O*(*N*)
    pub fn new(n: usize) -> Self {
        let data = Vec::from_iter(
            std::iter::repeat_with(<QueryProvider as Query<T>>::identity).take(n << 1),
        )
        .into_boxed_slice();

        Self {
            data,
            update_type: PhantomData,
            query_adapter: PhantomData,
            update_adapter: PhantomData,
        }
    }

    #[inline]
    fn inner_index(&self, i: usize) -> usize {
        self.data.len() / 2 + i
    }

    /// Returns `[l, r)` on `self.data`.
    #[inline]
    fn inner_range(&self, range: Range<usize>) -> [usize; 2] {
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

    /// Updates i-th data using `update`.
    ///
    /// # Example
    ///
    /// ```
    /// use seg_lib::{
    ///     normal::SegmentTree,
    ///     provider::{AddProvider, }
    /// };
    /// ```
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    pub fn point_update(&mut self, i: usize, update: U) {
        let mut i = self.inner_index(i);
        self.data[i] = <UpdateProvider as Update<U>>::update(&update, &self.data[i]);
        while i > 1 {
            i >>= 1;
            self.data[i] =
                <QueryProvider as Query<T>>::combine(&self.data[i * 2], &self.data[i * 2 + 1])
        }
    }

    /// Returns a shared reference ot i-th data.
    ///
    /// # Time complexity
    ///
    /// *O*(1)
    pub fn point_query(&self, i: usize) -> &T {
        let i = self.inner_index(i);
        &self.data[i]
    }

    /// Answers the query for the given range.
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    pub fn range_query(&self, range: Range<usize>) -> T {
        if range.is_empty() {
            return <QueryProvider as Query<T>>::identity();
        }

        let [mut l, mut r] = self.inner_range(range);
        // <=> l + 1 == r because l < r except when overflow occurs
        if l ^ r == 1 {
            return self.data[l].clone();
        }

        l >>= l.trailing_zeros();
        r >>= r.trailing_zeros();
        let (mut acc_l, mut acc_r) = (
            <QueryProvider as Query<T>>::identity(),
            <QueryProvider as Query<T>>::identity(),
        );
        while {
            if l > r {
                acc_l = <QueryProvider as Query<T>>::combine(&acc_l, &self.data[l]);
                l += 1;
                l >>= l.trailing_zeros()
            } else {
                r -= 1; // r > l >= 0
                acc_r = <QueryProvider as Query<T>>::combine(&self.data[r], &acc_r);
                r >>= r.trailing_zeros();
            }

            l != r
        } {}

        <QueryProvider as Query<T>>::combine(&acc_l, &acc_r)
    }
}

impl<T, U, QueryProvider, UpdateProvider> From<Vec<T>>
    for SegmentTree<T, U, QueryProvider, UpdateProvider>
where
    T: Clone,
    QueryProvider: Query<T>,
    UpdateProvider: Update<U, Arg = T>,
{
    /// Converts a [`Vec<T>`] into a [`SegmentTree<T, _, _, _>`].
    ///
    /// # Time Complexity
    ///
    /// *O*(*N*)
    fn from(values: Vec<T>) -> Self {
        let mut data = Vec::from_iter(
            std::iter::repeat_with(<QueryProvider as Query<T>>::identity)
                .take(values.len())
                .chain(values),
        )
        .into_boxed_slice();

        for i in (0..data.len() / 2).rev() {
            data[i] = <QueryProvider as Query<T>>::combine(&data[i * 2], &data[i * 2 + 1])
        }

        Self {
            data,
            update_type: PhantomData,
            query_adapter: PhantomData,
            update_adapter: PhantomData,
        }
    }
}

impl<T, U, QueryProvider, UpdateProvider> FromIterator<T>
    for SegmentTree<T, U, QueryProvider, UpdateProvider>
where
    T: Clone,
    QueryProvider: Query<T>,
    UpdateProvider: Update<U, Arg = T>,
{
    /// Creates a [`SegmentTree<T, _, _, _>`] from an iterator.
    ///
    /// # Time Complexity
    ///
    /// *O*(*N*)
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let (min, max) = iter.size_hint();
        if Some(min) == max {
            let mut data = Vec::from_iter(
                std::iter::repeat_with(<QueryProvider as Query<T>>::identity)
                    .take(min)
                    .chain(iter),
            )
            .into_boxed_slice();

            for i in data.len() / 2..data.len() {
                data[i] = <QueryProvider as Query<T>>::combine(&data[i * 2], &data[i * 2 + 1])
            }

            Self {
                data,
                update_type: PhantomData,
                query_adapter: PhantomData,
                update_adapter: PhantomData,
            }
        } else {
            Self::from(Vec::from_iter(iter))
        }
    }
}
