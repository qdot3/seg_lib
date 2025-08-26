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

    // for debug
    query: PhantomData<Query>,
}

impl<Query> AssignSegmentTree<Query>
where
    Query: Monoid,
    <Query as Monoid>::Set: Clone,
{
    const NULL_MAP_PTR: usize = !0;

    // /// Returns the number og elements.
    // ///
    // /// # Time complexity
    // ///
    // /// *O*(1)
    // #[allow(clippy::len_without_is_empty)]
    // pub fn len(&self) -> usize {
    //     self.lazy_ptr.len()
    // }

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

        // lazy propagation in top-to-bottom order
        for d in (l.trailing_zeros() + 1..usize::BITS - l.leading_zeros()).rev() {
            self.propagate_at(l >> d);
        }
        for d in (r.trailing_zeros() + 1..usize::BITS - r.leading_zeros()).rev() {
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
            for d in l.trailing_zeros() + 1..usize::BITS - l.leading_zeros() {
                self.recalculate_at(l >> d);
            }
            for d in r.trailing_zeros() + 1..usize::BITS - r.leading_zeros() {
                self.recalculate_at((r - 1) >> d);
            }
        } else {
            self.propagate_all();
            self.recalculate_all();
            self.lazy_map.clear();
        }
    }

    /// Answers query for the given range.
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    pub fn range_query<R>(&mut self, range: R) -> <Query as Monoid>::Set
    where
        R: RangeBounds<usize>,
    {
        let [l, r] = self.inner_range(range);

        // lazy propagation in top-to-bottom order
        for d in (l.trailing_zeros() + 1..usize::BITS - l.leading_zeros()).rev() {
            self.propagate_at(l >> d);
        }
        for d in (r.trailing_zeros() + 1..usize::BITS - r.leading_zeros()).rev() {
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
            query: PhantomData,
        };
        ast.recalculate_all();
        ast
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
            .field("query", &self.query)
            .finish()
    }
}
