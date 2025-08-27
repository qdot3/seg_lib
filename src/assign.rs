use std::{fmt::Debug, marker::PhantomData, ops::RangeBounds};

use crate::traits::Monoid;

/// A data structure that supports **range query range assign** operations.
pub struct AssignSegmentTree<Query>
where
    Query: Monoid,
    <Query as Monoid>::Set: Clone,
{
    /// the length is `buf_len + n | 1`
    data: Box<[<Query as Monoid>::Set]>,
    /// the length is `data.len() / 2`
    lazy_ptr: Box<[usize]>,
    /// vec![f,..., f^2i, ..., f^n.ilog2(), g, ..., g^2i, ..., g^n.ilog2(), h, ...]
    lazy_map: Vec<<Query as Monoid>::Set>,

    /// `n.next_power_of_2()`
    buf_len: usize,

    /// n
    data_len: usize,

    // for debug
    query: PhantomData<Query>,
}

impl<Query> AssignSegmentTree<Query>
where
    Query: Monoid,
    <Query as Monoid>::Set: Clone,
{
    const NULL_MAP_PTR: usize = !0;

    /// Cheats a new instance initialized with n [`Monoid::identity()`]s.
    ///
    /// # Time complexity
    ///
    /// *O*(*N*)
    pub fn new(n: usize) -> Self {
        Self::from_iter(std::iter::repeat_n(<Query as Monoid>::identity(), n))
    }

    /// Returns the number of elements.
    ///
    /// # Time complexity
    ///
    /// *O*(1)
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.data_len
    }

    /// Returns an iterator over the elements
    ///
    /// # Time complexity
    ///
    /// *O*(*N*)
    pub fn iter(&mut self) -> std::slice::Iter<'_, <Query as Monoid>::Set> {
        self.propagate_all();
        self.data[self.buf_len..self.buf_len + self.data_len].iter()
    }

    #[inline]
    fn inner_index(&self, i: usize) -> usize {
        self.data_len + i
    }

    /// Returns `[l, r)` on `self.data`.
    #[inline]
    fn inner_range<R>(&self, range: R) -> [usize; 2]
    where
        R: RangeBounds<usize>,
    {
        let l = match range.start_bound() {
            std::ops::Bound::Included(l) => self.buf_len + l,
            std::ops::Bound::Excluded(l) => self.buf_len + l + 1,
            std::ops::Bound::Unbounded => self.buf_len,
        };
        let r = match range.end_bound() {
            std::ops::Bound::Included(r) => self.buf_len + r + 1,
            std::ops::Bound::Excluded(r) => self.buf_len + r,
            std::ops::Bound::Unbounded => self.data.len(),
        };

        [l, r]
    }

    fn push_map(&mut self, i: usize, map_ptr: usize) {
        if map_ptr != Self::NULL_MAP_PTR {
            self.data[i] = self.lazy_map[map_ptr].clone();
            if let Some(prev) = self.lazy_ptr.get_mut(i) {
                *prev = map_ptr
            }
        }
    }

    /// Propagates pending updates for i-th node.
    ///
    /// # Panic
    ///
    /// Panics if the size of i-th segment is less than 2.
    fn propagate_at(&mut self, i: usize) {
        let map_ptr = std::mem::replace(&mut self.lazy_ptr[i], Self::NULL_MAP_PTR);
        if map_ptr != Self::NULL_MAP_PTR {
            self.push_map(i << 1, map_ptr - 1);
            self.push_map((i << 1) | 1, map_ptr - 1);
        }
    }

    /// Propagates all pending updates.
    fn propagate_all(&mut self) {
        for i in 1..self.data.len() >> 1 {
            self.propagate_at(i);
        }
    }

    /// Recalculates i-th node from the children
    ///
    /// # Panics
    ///
    /// Panics if the size of i-th segment is less than 2.
    #[inline]
    fn recalculate_at(&mut self, i: usize) {
        self.data[i] = <Query as Monoid>::combine(&self.data[i << 1], &self.data[(i << 1) | 1])
    }

    /// Recalculates all data segments.
    fn recalculate_all(&mut self) {
        for i in (1..self.data.len() >> 1).rev() {
            self.recalculate_at(i);
        }
    }

    /// Assigns the element in the range.
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    pub fn range_assign<R>(&mut self, range: R, element: <Query as Monoid>::Set)
    where
        R: RangeBounds<usize>,
    {
        let [l, r] = self.inner_range(range);
        if l >= r {
            return;
        }
        // if l ^ r == 1 {
        //     self.point_assign(l - self.buf_len, element);
        //     return;
        // }

        // lazy propagation in top-to-bottom order
        let diff = usize::BITS - (l ^ (r - 1)).leading_zeros();
        for d in (diff + 1..=self.buf_len.trailing_zeros()).rev() {
            self.propagate_at(l >> d);
        }
        for d in (l.trailing_zeros() + 1..=diff).rev() {
            self.propagate_at(l >> d);
        }
        for d in (r.trailing_zeros() + 1..=diff).rev() {
            self.propagate_at((r - 1) >> d);
        }

        // assign new element
        {
            let mut pow = element;
            let [mut l, mut r] = [l, r];
            while l < r {
                self.lazy_map.push(pow.clone());

                if l & 1 == 1 {
                    self.push_map(l, self.lazy_map.len() - 1);
                    l += 1;
                }
                if r & 1 == 1 {
                    r -= 1;
                    self.push_map(r, self.lazy_map.len() - 1);
                }

                l >>= 1;
                r >>= 1;
                pow = <Query as Monoid>::combine(&pow, &pow)
            }

            debug_assert_eq!(l, r);
            while l > 1 {
                l >>= 1;
                self.lazy_map.push(pow.clone());
                pow = <Query as Monoid>::combine(&pow, &pow)
            }
        }

        if self.lazy_map.len() < self.buf_len {
            // recalculate data segments in bottom-to-top order
            let diff = usize::BITS - (l ^ (r - 1)).leading_zeros();
            for d in l.trailing_zeros() + 1..=diff {
                self.recalculate_at(l >> d);
            }
            for d in r.trailing_zeros() + 1..=diff {
                self.recalculate_at((r - 1) >> d);
            }
            for d in diff + 1..=self.buf_len.trailing_zeros() {
                self.recalculate_at(l >> d);
            }
        } else {
            self.propagate_all();
            self.recalculate_all();
            self.lazy_map.clear();
        }
    }

    /// Assign the element to i-th node.
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    pub fn point_assign(&mut self, i: usize, element: <Query as Monoid>::Set) {
        let i = self.inner_index(i);

        // lazy propagation in top-to-bottom order
        for d in (1..=self.buf_len.trailing_zeros()).rev() {
            self.propagate_at(i >> d);
        }

        // assign new element
        self.data[i] = element;

        // recalculate data segments in bottom-to-top order
        for d in 1..=self.buf_len.trailing_zeros() {
            self.recalculate_at(i >> d);
        }
    }

    /// Answers query for the given range.
    ///
    /// If the given range is empty, returns [`Monoid::identity()`].
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    pub fn range_query<R>(&mut self, range: R) -> <Query as Monoid>::Set
    where
        R: RangeBounds<usize>,
    {
        let [l, r] = self.inner_range(range);
        if l >= r {
            return <Query as Monoid>::identity();
        }
        // if l ^ r == 1 {
        //     return self.point_query(l - self.buf_len).clone();
        // }

        // lazy propagation in top-to-bottom order
        let diff = usize::BITS - (l ^ (r - 1)).leading_zeros();
        for d in (diff + 1..=self.buf_len.trailing_zeros()).rev() {
            self.propagate_at(l >> d);
        }
        for d in (l.trailing_zeros() + 1..=diff).rev() {
            self.propagate_at(l >> d);
        }
        for d in (r.trailing_zeros() + 1..=diff).rev() {
            self.propagate_at((r - 1) >> d);
        }

        let [mut l, mut r] = [l >> l.trailing_zeros(), r >> r.trailing_zeros()];
        let [mut acc_l, mut acc_r] = [<Query as Monoid>::identity(), <Query as Monoid>::identity()];
        while {
            if l >= r {
                acc_l = <Query as Monoid>::combine(&acc_l, &self.data[l]);
                l += 1;
                l >>= l.trailing_zeros();
            } else {
                r -= 1;
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
    /// *O*(log *N*)
    pub fn point_query(&mut self, i: usize) -> &<Query as Monoid>::Set {
        let i = self.inner_index(i);

        // lazy propagation in top-to-bottom order
        for d in (1..=self.buf_len.trailing_zeros()).rev() {
            self.propagate_at(i >> d);
        }

        &self.data[i]
    }
}

impl<Query> From<Vec<<Query as Monoid>::Set>> for AssignSegmentTree<Query>
where
    Query: Monoid,
    <Query as Monoid>::Set: Clone,
{
    fn from(values: Vec<<Query as Monoid>::Set>) -> Self {
        let n = values.len();
        let buf_len = n.next_power_of_two();

        let data = Vec::from_iter(
            std::iter::repeat_with(<Query as Monoid>::identity)
                .take(buf_len)
                .chain(values)
                .chain(std::iter::repeat_with(<Query as Monoid>::identity).take(n & 1)),
        )
        .into_boxed_slice();

        let mut ast = Self {
            data,
            lazy_ptr: vec![Self::NULL_MAP_PTR; (buf_len + n + 1) >> 1].into_boxed_slice(),
            lazy_map: Vec::with_capacity(buf_len + (n | 1).ilog2() as usize),
            buf_len,
            data_len: n,
            query: PhantomData,
        };
        ast.recalculate_all();
        ast
    }
}

impl<Query> FromIterator<<Query as Monoid>::Set> for AssignSegmentTree<Query>
where
    Query: Monoid,
    <Query as Monoid>::Set: Clone,
{
    fn from_iter<T: IntoIterator<Item = <Query as Monoid>::Set>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let (min, max) = iter.size_hint();
        if Some(min) == max {
            let buf_len = min.next_power_of_two();

            let data = Vec::from_iter(
                std::iter::repeat_with(<Query as Monoid>::identity)
                    .take(buf_len)
                    .chain(iter)
                    .chain(std::iter::repeat_with(<Query as Monoid>::identity).take(min & 1)),
            )
            .into_boxed_slice();

            let mut ast = Self {
                data,
                lazy_ptr: vec![Self::NULL_MAP_PTR; (buf_len + min + 1) >> 1].into_boxed_slice(),
                lazy_map: Vec::with_capacity(buf_len + (min | 1).ilog2() as usize),
                buf_len,
                data_len: min,
                query: PhantomData,
            };
            ast.recalculate_all();
            ast
        } else {
            Self::from(Vec::from_iter(iter))
        }
    }
}

impl<Query> Debug for AssignSegmentTree<Query>
where
    Query: Monoid,
    <Query as Monoid>::Set: Clone + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AssignSegmentTree")
            .field("data", &self.data)
            .field("lazy_ptr", &self.lazy_ptr)
            .field("lazy_map", &self.lazy_map)
            .field("buf_len", &self.buf_len)
            .field("data_len", &self.data_len)
            .field("query", &self.query)
            .finish()
    }
}

impl<Query> Clone for AssignSegmentTree<Query>
where
    Query: Monoid,
    <Query as Monoid>::Set: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            lazy_ptr: self.lazy_ptr.clone(),
            lazy_map: self.lazy_map.clone(),
            buf_len: self.buf_len.clone(),
            data_len: self.data_len.clone(),
            query: self.query,
        }
    }
}
