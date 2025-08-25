use std::{fmt::Debug, marker::PhantomData, ops::RangeBounds};

use crate::traits::{Monoid, MonoidAction};

/// A data structure that supports *range query range update* operation.
pub struct LazySegmentTree<Query, Update>
where
    Query: Monoid,
    Update: Monoid + MonoidAction<Map = Update, Set = Query>,
{
    data: Box<[<Query as Monoid>::Set]>,
    lazy: Box<[<Update as Monoid>::Set]>,

    segment_size: Option<Box<[usize]>>,

    // for debug
    query: PhantomData<Query>,
    update: PhantomData<Update>,
}

impl<Query, Update> LazySegmentTree<Query, Update>
where
    Query: Monoid,
    Update: Monoid + MonoidAction<Map = Update, Set = Query>,
{
    /// Cheats an instance initialized with `n` [`Monoid::identity()`]s.
    ///
    /// # Time complexity
    ///
    /// *O*(*N*)
    pub fn new(n: usize) -> Self {
        Self::from_iter(std::iter::repeat_with(<Query as Monoid>::identity).take(n))
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

    /// Evaluates pending updates of i-th segment.
    fn eval(&mut self, i: usize) -> <Query as Monoid>::Set {
        let size = self
            .segment_size
            .as_ref()
            .map(|segment_size| segment_size[i]);
        <Update as MonoidAction>::act(&self.lazy[i], &self.data[i], size)
    }

    /// Propagates pending [`Monoid::combine`] operations to the children.
    ///
    /// # Panics
    ///
    /// Panics if either of children does **not** exist.
    fn propagate(&mut self, i: usize) {
        self.data[i] = self.eval(i);

        let mapping = std::mem::replace(&mut self.lazy[i], <Update as Monoid>::identity());
        self.lazy[i << 1] = <Update as Monoid>::combine(&self.lazy[i << 1], &mapping);
        self.lazy[(i << 1) | 1] = <Update as Monoid>::combine(&self.lazy[(i << 1) | 1], &mapping);
    }

    /// Recalculates i-th data segments from the children.
    ///
    /// # Panics
    ///
    /// Panics if either of children does **not** exist.
    #[inline]
    fn recalculate(&mut self, i: usize) {
        self.data[i] = <Query as Monoid>::combine(&self.eval(i << 1), &self.eval((i << 1) | 1))
    }

    /// Updates elements in the range using [`Monoid::combine()`].
    /// More precisely, `a[i] <- update · a[i], i ∈ range`
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    pub fn range_update<R>(&mut self, range: R, update: <Update as Monoid>::Set)
    where
        R: RangeBounds<usize>,
    {
        let [l, r] = self.inner_range(range);

        if l >= r {
            return;
        }

        // lazy propagation in bottom-to-top order
        if !<Update as Monoid>::IS_COMMUTATIVE {
            for d in (l.trailing_zeros() + 1..usize::BITS - l.leading_zeros()).rev() {
                self.propagate(l >> d);
            }
            for d in (r.trailing_zeros() + 1..usize::BITS - r.leading_zeros()).rev() {
                self.propagate((r - 1) >> d);
            }
        }

        // push the given update to corresponding lazy segments
        {
            let [mut l, mut r] = [l >> l.trailing_zeros(), r >> r.trailing_zeros()];
            while {
                if l >= r {
                    self.lazy[l] = <Update as Monoid>::combine(&self.lazy[l], &update);
                    l += 1;
                    l >>= l.trailing_zeros();
                } else {
                    r -= 1;
                    self.lazy[r] = <Update as Monoid>::combine(&self.lazy[r], &update);
                    r >>= r.trailing_zeros();
                }

                l != r
            } {}
        }

        // recalculate data segments in bottom-to-top order
        for d in l.trailing_zeros() + 1..usize::BITS - l.leading_zeros() {
            self.recalculate(l >> d);
        }
        for d in r.trailing_zeros() + 1..usize::BITS - r.leading_zeros() {
            self.recalculate((r - 1) >> d);
        }
    }

    /// Updates i-th element in the range using [`Monoid::combine()`].
    /// More precisely, `a[i] <- update · a[i]`.
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    pub fn point_update(&mut self, i: usize, update: <Update as Monoid>::Set) {
        let i = self.inner_index(i);

        // lazy propagation
        if !<Update as Monoid>::IS_COMMUTATIVE {
            for d in (i.trailing_zeros() + 1..usize::BITS - i.leading_zeros()).rev() {
                self.propagate(i >> d);
            }
        }

        self.lazy[i] = <Update as Monoid>::combine(&self.lazy[i], &update);

        // recalculate
        for d in i.trailing_zeros() + 1..usize::BITS - i.leading_zeros() {
            self.recalculate(i >> d);
        }
    }

    /// Answers the query for the given range.
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

        // lazy propagation
        for d in (l.trailing_zeros() + 1..usize::BITS - l.leading_zeros()).rev() {
            self.propagate(l >> d);
        }
        for d in (r.trailing_zeros() + 1..usize::BITS - r.leading_zeros()).rev() {
            self.propagate((r - 1) >> d);
        }

        // reflect pending updates and combine segments
        let [mut l, mut r] = [l >> l.trailing_zeros(), r >> r.trailing_zeros()];
        let [mut acc_l, mut acc_r] = [<Query as Monoid>::identity(), <Query as Monoid>::identity()];
        while {
            if l >= r {
                acc_l = <Query as Monoid>::combine(&acc_l, &self.eval(l));
                l += 1;
                l >>= l.trailing_zeros();
            } else {
                r -= 1;
                acc_r = <Query as Monoid>::combine(&self.eval(r), &acc_r);
                r >>= r.trailing_zeros();
            }

            l != r
        } {}

        <Query as Monoid>::combine(&acc_l, &acc_r)
    }

    /// Answers the query for i-th element.
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    pub fn point_query(&mut self, i: usize) -> &<Query as Monoid>::Set {
        let i = self.inner_index(i);

        // lazy propagation
        for d in (i.trailing_zeros() + 1..usize::BITS - i.leading_zeros()).rev() {
            self.propagate(i >> d);
        }

        &self.data[i]
    }
}

impl<Query, Update> From<Vec<<Query as Monoid>::Set>> for LazySegmentTree<Query, Update>
where
    Query: Monoid,
    Update: Monoid + MonoidAction<Map = Update, Set = Query>,
{
    fn from(values: Vec<<Query as Monoid>::Set>) -> Self {
        let n = values.len();

        let mut data = Vec::from_iter(
            std::iter::repeat_with(<Query as Monoid>::identity)
                .take(n)
                .chain(values),
        )
        .into_boxed_slice();
        for i in (1..n).rev() {
            data[i] = <Query as Monoid>::combine(&data[i << 1], &data[(i << 1) | 1])
        }

        let lazy =
            Vec::from_iter(std::iter::repeat_with(<Update as Monoid>::identity).take(n << 1))
                .into_boxed_slice();

        let segment_size = <Update as MonoidAction>::USE_SEGMENT_SIZE.then(|| {
            let mut segment_size =
                Vec::from_iter(std::iter::repeat_n(0, n).chain(std::iter::repeat_n(1, n)))
                    .into_boxed_slice();
            for i in (1..n).rev() {
                segment_size[i] = segment_size[i << 1] + segment_size[(i << 1) | 1]
            }

            segment_size
        });

        Self {
            data,
            lazy,
            segment_size,
            query: PhantomData,
            update: PhantomData,
        }
    }
}

impl<Query, Update> FromIterator<<Query as Monoid>::Set> for LazySegmentTree<Query, Update>
where
    Query: Monoid,
    Update: Monoid + MonoidAction<Map = Update, Set = Query>,
{
    fn from_iter<T: IntoIterator<Item = <Query as Monoid>::Set>>(iter: T) -> Self {
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
                data[i] = <Query as Monoid>::combine(&data[i << 1], &data[(i << 1) | 1])
            }

            let lazy =
                Vec::from_iter(std::iter::repeat_with(<Update as Monoid>::identity).take(min << 1))
                    .into_boxed_slice();

            let segment_size = <Update as MonoidAction>::USE_SEGMENT_SIZE.then(|| {
                let mut segment_size =
                    Vec::from_iter(std::iter::repeat_n(0, min).chain(std::iter::repeat_n(1, min)))
                        .into_boxed_slice();
                for i in (1..min).rev() {
                    segment_size[i] = segment_size[i << 1] + segment_size[(i << 1) | 1]
                }

                segment_size
            });

            Self {
                data,
                lazy,
                segment_size,
                query: PhantomData,
                update: PhantomData,
            }
        } else {
            Vec::from_iter(iter).into()
        }
    }
}

impl<Query, Update> Debug for LazySegmentTree<Query, Update>
where
    Query: Monoid,
    <Query as Monoid>::Set: Debug,
    Update: Monoid + MonoidAction<Map = Update, Set = Query>,
    <Update as Monoid>::Set: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LazySegmentTree")
            .field("data", &self.data)
            .field("lazy", &self.lazy)
            .field("segment_size", &self.segment_size)
            .field("query", &self.query)
            .field("update", &self.update)
            .finish()
    }
}

impl<Query, Update> Clone for LazySegmentTree<Query, Update>
where
    Query: Monoid,
    <Query as Monoid>::Set: Clone,
    Update: Monoid + MonoidAction<Map = Update, Set = Query>,
    <Update as Monoid>::Set: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            lazy: self.lazy.clone(),
            segment_size: self.segment_size.clone(),
            query: self.query,
            update: self.update,
        }
    }
}
