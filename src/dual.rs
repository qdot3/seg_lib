use std::{fmt::Debug, marker::PhantomData, ops::RangeBounds};

use crate::traits::Monoid;

/// A data structure that supports **point query range update** operations.
pub struct DualSegmentTree<Update>
where
    Update: Monoid,
{
    data: Box<[<Update as Monoid>::Set]>,
    /// for Debug
    update: PhantomData<Update>,
}

impl<Update> DualSegmentTree<Update>
where
    Update: Monoid,
{
    /// Creates a new instance initialized with `n` [identity elements](crate::traits::Monoid::identity()).
    ///
    /// # Time complexity
    ///
    /// *O*(*N*)
    pub fn new(n: usize) -> Self {
        let data =
            Vec::from_iter(std::iter::repeat_with(<Update as Monoid>::identity).take(n << 1))
                .into_boxed_slice();

        Self {
            data,
            update: PhantomData,
        }
    }

    /// Returns the number of elements.
    ///
    /// # Time complexity
    ///
    /// *O*(1)
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
    pub fn iter(&mut self) -> std::slice::Iter<'_, <Update as Monoid>::Set> {
        self.propagate_all();
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

    /// Performs pending [`Monoid::combine`] operations for `i`-th node.
    ///
    /// # Panics
    ///
    /// Panics if either of children does **not** exist.
    fn propagate_at(&mut self, i: usize) {
        assert!(
            self.data.len() >= (i << 1) + 2,
            "{i}-th node should have two children",
        );

        let update = std::mem::replace(&mut self.data[i], <Update as Monoid>::identity());
        self.data[i << 1] = <Update as Monoid>::combine(&self.data[i << 1], &update);
        self.data[(i << 1) | 1] = <Update as Monoid>::combine(&self.data[(i << 1) | 1], &update);

        // let children = &mut self.data[i << 1..(i << 1) + 2];
        // children[0] = <Update as Monoid>::combine(&lazy, &children[0]);
        // children[1] = <Update as Monoid>::combine(&lazy, &children[1]);
    }

    fn propagate_all(&mut self) {
        for i in 1..self.data.len() >> 1 {
            self.propagate_at(i);
        }
    }

    /// Updates elements in the range using [predefined binary operation](crate::traits::Monoid::combine()).
    /// More precisely, `a[i] <- update · a[i], i ∈ range`
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    pub fn range_update<R>(&mut self, range: R, update: &<Update as Monoid>::Set)
    where
        R: RangeBounds<usize>,
    {
        let [l, r] = self.inner_range(range);

        if l >= r {
            return;
        }
        // // l + 1 == r
        // if l ^ r == 1 {
        //     self.point_update(l, update);
        //     return;
        // }

        let [l, r] = [l >> l.trailing_zeros(), r >> r.trailing_zeros()];
        // lazy propagation in top-to-bottom order
        if !<Update as Monoid>::IS_COMMUTATIVE {
            for d in (1..usize::BITS - l.leading_zeros()).rev() {
                self.propagate_at(l >> d);
            }
            for d in (1..usize::BITS - r.leading_zeros()).rev() {
                self.propagate_at(r >> d);
            }
        }

        let [mut l, mut r] = [l, r];
        while {
            if l >= r {
                self.data[l] = <Update as Monoid>::combine(&self.data[l], update);
                l += 1;
                l >>= l.trailing_zeros();
            } else {
                r -= 1;
                self.data[r] = <Update as Monoid>::combine(&self.data[r], update);
                r >>= r.trailing_zeros()
            }

            l != r
        } {}
    }

    /// Updates `i`-th element using [predefined binary operation](crate::traits::Monoid::combine()).
    /// More precisely, `a[i] <- update · a[i]`
    ///
    /// # Time complexity
    ///
    /// | [commutativity](crate::traits::Monoid::IS_COMMUTATIVE) | time         |
    /// |--------------------------------------------------------|--------------|
    /// | [`true`]                                               | *O*(1)       |
    /// | [`false`]                                              | *O*(log *N*) |
    pub fn point_update(&mut self, i: usize, update: &<Update as Monoid>::Set) {
        let i = self.inner_index(i);

        // lazy propagation in top-to-bottom order
        if !<Update as Monoid>::IS_COMMUTATIVE {
            for d in (1..usize::BITS - i.leading_zeros()).rev() {
                self.propagate_at(i >> d);
            }
        }

        self.data[i] = <Update as Monoid>::combine(&self.data[i], update);
    }

    /// Returns `i`-th element.
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    pub fn point_query(&self, i: usize) -> <Update as Monoid>::Set {
        let mut i = self.inner_index(i);
        let mut res = <Update as Monoid>::identity();
        // combine in chronological order
        while i > 0 {
            res = <Update as Monoid>::combine(&self.data[i], &res);
            i >>= 1;
        }

        res
    }

    /// Returns modified i-th element using `f`.
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    pub fn point_query_with<F, T>(&self, i: usize, f: F) -> T
    where
        F: FnOnce(<Update as Monoid>::Set) -> T,
    {
        f(self.point_query(i))
    }
}

impl<Update> From<Vec<<Update as Monoid>::Set>> for DualSegmentTree<Update>
where
    Update: Monoid,
{
    fn from(values: Vec<<Update as Monoid>::Set>) -> Self {
        let data = Vec::from_iter(
            std::iter::repeat_with(<Update as Monoid>::identity)
                .take(values.len())
                .chain(values),
        )
        .into_boxed_slice();

        Self {
            data,
            update: PhantomData,
        }
    }
}

impl<Update> FromIterator<<Update as Monoid>::Set> for DualSegmentTree<Update>
where
    Update: Monoid,
{
    fn from_iter<I: IntoIterator<Item = <Update as Monoid>::Set>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let (min, max) = iter.size_hint();
        if Some(min) == max {
            let data = Vec::from_iter(
                std::iter::repeat_with(<Update as Monoid>::identity)
                    .take(min)
                    .chain(iter),
            )
            .into_boxed_slice();

            Self {
                data,
                update: PhantomData,
            }
        } else {
            Vec::from_iter(iter).into()
        }
    }
}

impl<Update> Debug for DualSegmentTree<Update>
where
    Update: Monoid,
    <Update as Monoid>::Set: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DualSegmentTree")
            .field("data", &self.data)
            .field("update", &self.update)
            .finish()
    }
}

impl<Update> Clone for DualSegmentTree<Update>
where
    Update: Monoid,
    <Update as Monoid>::Set: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            update: self.update,
        }
    }
}

#[cfg(test)]
mod range_update {
    use rand::Rng;

    use crate::{dual::DualSegmentTree, ops::Add};

    fn template(n: usize) {
        let mut dual = DualSegmentTree::<Add<usize>>::new(n);

        for l in 0..n {
            for r in 0..n {
                dual.range_update(l..=r, &1);
            }
        }

        for i in 0..n {
            let result = dual.point_query(i);
            let expected = (i + 1) * (n - i);
            assert_eq!(result, expected, "panics when n = {n}, i = {i}, {dual:?}",);
        }
    }

    #[test]
    fn test_pow_2() {
        for d in 0..5 {
            template(1 << d);
        }
    }

    #[test]
    fn test_random() {
        let mut rng = rand::rng();
        for _ in 0..10 {
            let n = rng.random_range(100..5_000);
            template(n);
        }
    }
}
