use std::{fmt::Debug, ops::RangeBounds};

use crate::traits::{Monoid, MonoidAction};

/// A data structure that supports **range query range update** operations.
// ANCHOR: definition
pub struct LazySegmentTree<Action>
where
    Action: MonoidAction,
{
    data: Box<[<<Action as MonoidAction>::Set as Monoid>::Set]>,
    lazy: Box<[<<Action as MonoidAction>::Map as Monoid>::Set]>,

    /// calculate if [`MonoidAction::USE_SEGMENT_SIZE`] is `true`.
    segment_size: Option<Box<[usize]>>,
}
// ANCHOR_END: definition

impl<Action> LazySegmentTree<Action>
where
    Action: MonoidAction,
{
    /// Creates a new instance initialized with `n` [identity elements](crate::traits::Monoid::identity()).
    ///
    /// # Time complexity
    ///
    /// *O*(*N*)
    ///
    /// # Example
    ///
    /// ```
    /// use seg_lib::{LazySegmentTree, acts::MaxQueryAddUpdate};
    ///
    /// let lst = LazySegmentTree::<MaxQueryAddUpdate<i32>>::new(100);
    /// ```
    #[inline]
    pub fn new(n: usize) -> Self {
        Self::from_iter(
            std::iter::repeat_with(<<Action as MonoidAction>::Set as Monoid>::identity).take(n),
        )
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
    /// use seg_lib::{LazySegmentTree, acts::MaxQueryAddUpdate};
    ///
    /// let lst = LazySegmentTree::<MaxQueryAddUpdate<i32>>::new(100);
    /// assert_eq!(lst.len(), 100);
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
    /// use seg_lib::{LazySegmentTree, acts::MaxQueryAddUpdate};
    ///
    /// let mut lst = LazySegmentTree::<MaxQueryAddUpdate<i32>>::new(100);
    /// assert!(lst.iter().all(|v| v.is_none()));
    /// ```
    #[inline]
    pub fn iter(&mut self) -> std::slice::Iter<'_, <<Action as MonoidAction>::Set as Monoid>::Set> {
        self.propagate_all();
        self.recalculate_all();
        self.data[self.data.len() >> 1..].iter()
    }

    #[inline]
    fn inner_index(&self, i: usize) -> usize {
        self.data.len() / 2 + i
    }

    /// Returns `[l, r)` on `self.data`.
    #[inline]
    fn translate_range<R>(&self, range: R) -> [usize; 2]
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

        [l, r]
    }

    fn push_map(&mut self, i: usize, update: &<<Action as MonoidAction>::Map as Monoid>::Set) {
        let size = self
            .segment_size
            .as_ref()
            .map(|segment_size| segment_size.get(i).copied().unwrap_or(1));
        self.data[i] = <Action as MonoidAction>::act(update, &self.data[i], size);

        if let Some(lazy) = self.lazy.get_mut(i) {
            *lazy = <<Action as MonoidAction>::Map as Monoid>::combine(lazy, update)
        }
    }

    /// Propagates pending [`Monoid::combine`] operations to the children.
    ///
    /// # Panics
    ///
    /// Panics if either of children does **not** exist.
    fn propagate_at(&mut self, i: usize) {
        let mapping = std::mem::replace(
            &mut self.lazy[i],
            <<Action as MonoidAction>::Map as Monoid>::identity(),
        );
        self.push_map(i << 1, &mapping);
        self.push_map((i << 1) | 1, &mapping);
    }

    fn propagate_all(&mut self) {
        for i in 1..self.data.len() >> 1 {
            self.propagate_at(i);
        }
    }

    /// Recalculates i-th data segments from the children.
    ///
    /// # Panics
    ///
    /// Panics if either of children does **not** exist.
    #[inline]
    fn recalculate_at(&mut self, i: usize) {
        self.data[i] = <<Action as MonoidAction>::Set as Monoid>::combine(
            &self.data[i << 1],
            &self.data[(i << 1) | 1],
        )
    }

    /// Recalculates all data segments.
    fn recalculate_all(&mut self) {
        for i in (1..self.data.len() >> 1).rev() {
            self.recalculate_at(i);
        }
    }

    /// Updates elements in the range using [predefined binary operation](crate::traits::Monoid::combine()).
    /// More precisely, `a[i] <- update · a[i], i ∈ range`
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    ///
    /// # Example
    ///
    /// ```
    /// use seg_lib::{LazySegmentTree, acts::MaxQueryAddUpdate};
    ///
    /// let mut lst = LazySegmentTree::<MaxQueryAddUpdate<i32>>::from_iter(
    ///     std::iter::repeat_n(Some(0), 100)
    /// );
    /// assert_eq!(lst.range_query(..), Some(0));
    ///
    /// lst.range_update(..75, &100);
    /// lst.range_update(25.., &110);
    /// assert_eq!(lst.range_query(..25), Some(100));
    /// assert_eq!(lst.range_query(..), Some(210));
    /// assert_eq!(lst.range_query(75..), Some(110));
    /// ```
    pub fn range_update<R>(
        &mut self,
        range: R,
        update: &<<Action as MonoidAction>::Map as Monoid>::Set,
    ) where
        R: RangeBounds<usize>,
    {
        let [l, r] = {
            let [l, r] = self.translate_range(range);
            if l >= r {
                return;
            }
            if l + 1 == r {
                self.point_update(l, update);
                return;
            }

            [self.inner_index(l), self.inner_index(r)]
        };

        // lazy propagation in bottom-to-top order
        if !<<Action as MonoidAction>::Map as Monoid>::IS_COMMUTATIVE {
            let diff = usize::BITS - (l ^ (r - 1)).leading_zeros();
            for d in (diff + 1..usize::BITS - l.leading_zeros()).rev() {
                self.propagate_at(l >> d);
            }
            for d in (l.trailing_zeros() + 1..=diff).rev() {
                self.propagate_at(l >> d);
            }
            for d in (r.trailing_zeros() + 1..=diff).rev() {
                self.propagate_at((r - 1) >> d);
            }
        }

        // push the given update to corresponding lazy segments
        {
            let [mut l, mut r] = [l >> l.trailing_zeros(), r >> r.trailing_zeros()];
            while {
                if l >= r {
                    self.push_map(l, update);
                    l += 1;
                    l >>= l.trailing_zeros();
                } else {
                    r -= 1;
                    self.push_map(r, update);
                    r >>= r.trailing_zeros();
                }

                l != r
            } {}
        }

        // recalculate data segments in bottom-to-top order
        let diff = usize::BITS - (l ^ (r - 1)).leading_zeros();
        for d in l.trailing_zeros() + 1..=diff {
            self.recalculate_at(l >> d);
        }
        for d in r.trailing_zeros() + 1..=diff {
            self.recalculate_at((r - 1) >> d);
        }
        for d in diff + 1..usize::BITS - l.leading_zeros() {
            self.recalculate_at(l >> d);
        }
    }

    /// Updates i-th element using [predefined binary operation](crate::traits::Monoid::combine()).
    /// More precisely, `a[i] <- update · a[i]`.
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    ///
    /// # Example
    ///
    /// ```
    /// use seg_lib::{LazySegmentTree, acts::MaxQueryAddUpdate};
    ///
    /// let mut lst = LazySegmentTree::<MaxQueryAddUpdate<i32>>::from_iter(
    ///     std::iter::repeat_n(Some(0), 100)
    /// );
    /// assert_eq!(lst.range_query(..), Some(0));
    ///
    /// lst.point_update(25, &10);
    /// lst.point_update(75, &20);
    /// assert_eq!(lst.range_query(..), Some(20));
    /// assert_eq!(lst.range_query(..50), Some(10))
    /// ```
    pub fn point_update(
        &mut self,
        i: usize,
        update: &<<Action as MonoidAction>::Map as Monoid>::Set,
    ) {
        let i = self.inner_index(i);

        // lazy propagation
        if !<<Action as MonoidAction>::Map as Monoid>::IS_COMMUTATIVE {
            for d in (1..usize::BITS - i.leading_zeros()).rev() {
                self.propagate_at(i >> d);
            }
        }

        self.push_map(i, update);

        // recalculate
        for d in 1..usize::BITS - i.leading_zeros() {
            self.recalculate_at(i >> d);
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
    /// use seg_lib::{LazySegmentTree, acts::MaxQueryAddUpdate};
    ///
    /// let mut lst = LazySegmentTree::<MaxQueryAddUpdate<i32>>::from_iter(
    ///     (0..100).map(|v| Some(v))
    /// );
    /// assert_eq!(lst.range_query(..), Some(99));
    /// assert_eq!(lst.range_query(..50), Some(49));
    /// assert_eq!(lst.range_query(..=50), Some(50));
    /// assert_eq!(lst.range_query(50..), Some(99));
    /// ```
    pub fn range_query<R>(&mut self, range: R) -> <<Action as MonoidAction>::Set as Monoid>::Set
    where
        R: RangeBounds<usize>,
    {
        let [l, r] = {
            let [l, r] = self.translate_range(range);
            if l >= r {
                return <<Action as MonoidAction>::Set as Monoid>::identity();
            }
            if l + 1 == r {
                return <<Action as MonoidAction>::Set as Monoid>::combine(
                    self.point_query(l),
                    &<<Action as MonoidAction>::Set as Monoid>::identity(),
                );
            }
            [self.inner_index(l), self.inner_index(r)]
        };

        // lazy propagation
        let diff = usize::BITS - (l ^ (r - 1)).leading_zeros();
        for d in (diff + 1..usize::BITS - l.leading_zeros()).rev() {
            self.propagate_at(l >> d);
        }
        for d in (l.trailing_zeros() + 1..=diff).rev() {
            self.propagate_at(l >> d);
        }
        for d in (r.trailing_zeros() + 1..=diff).rev() {
            self.propagate_at((r - 1) >> d);
        }

        // reflect pending updates and combine segments
        let [mut l, mut r] = [l >> l.trailing_zeros(), r >> r.trailing_zeros()];
        let [mut acc_l, mut acc_r] = [
            <<Action as MonoidAction>::Set as Monoid>::identity(),
            <<Action as MonoidAction>::Set as Monoid>::identity(),
        ];
        while {
            if l >= r {
                acc_l = <<Action as MonoidAction>::Set as Monoid>::combine(&acc_l, &self.data[l]);
                l += 1;
                l >>= l.trailing_zeros();
            } else {
                r -= 1;
                acc_r = <<Action as MonoidAction>::Set as Monoid>::combine(&self.data[r], &acc_r);
                r >>= r.trailing_zeros();
            }

            l != r
        } {}

        <<Action as MonoidAction>::Set as Monoid>::combine(&acc_l, &acc_r)
    }

    /// Answers query for i-th element.
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    ///
    /// # Example
    ///
    /// ```
    /// use seg_lib::{LazySegmentTree, acts::MaxQueryAddUpdate};
    ///
    /// let mut lst = LazySegmentTree::<MaxQueryAddUpdate<i32>>::from_iter(
    ///     (0..100).map(|v| Some(v))
    /// );
    /// 
    /// for i in 0..100 {
    ///     assert_eq!(lst.point_query(i), &Some(i as i32))
    /// }
    /// ```
    pub fn point_query(&mut self, i: usize) -> &<<Action as MonoidAction>::Set as Monoid>::Set {
        let i = self.inner_index(i);

        // lazy propagation
        for d in (1..usize::BITS - i.leading_zeros()).rev() {
            self.propagate_at(i >> d);
        }

        &self.data[i]
    }
}

impl<Action> From<Vec<<<Action as MonoidAction>::Set as Monoid>::Set>> for LazySegmentTree<Action>
where
    Action: MonoidAction,
{
    fn from(values: Vec<<<Action as MonoidAction>::Set as Monoid>::Set>) -> Self {
        let n = values.len();

        let data = Vec::from_iter(
            std::iter::repeat_with(<<Action as MonoidAction>::Set as Monoid>::identity)
                .take(n)
                .chain(values),
        )
        .into_boxed_slice();

        let lazy = Vec::from_iter(
            std::iter::repeat_with(<<Action as MonoidAction>::Map as Monoid>::identity).take(n),
        )
        .into_boxed_slice();

        let segment_size = <Action as MonoidAction>::USE_SEGMENT_SIZE.then(|| {
            let mut segment_size =
                Vec::from_iter(std::iter::repeat_n(0, n).chain(std::iter::repeat_n(1, n)));
            for i in (1..n).rev() {
                segment_size[i] = segment_size[i << 1] + segment_size[(i << 1) | 1]
            }
            segment_size.truncate(n);

            segment_size.into_boxed_slice()
        });

        let mut lst = Self {
            data,
            lazy,
            segment_size,
        };
        lst.recalculate_all();
        lst
    }
}

impl<Action> FromIterator<<<Action as MonoidAction>::Set as Monoid>::Set>
    for LazySegmentTree<Action>
where
    Action: MonoidAction,
{
    fn from_iter<T: IntoIterator<Item = <<Action as MonoidAction>::Set as Monoid>::Set>>(
        iter: T,
    ) -> Self {
        let iter = iter.into_iter();
        let (min, max) = iter.size_hint();
        if Some(min) == max {
            let data = Vec::from_iter(
                std::iter::repeat_with(<<Action as MonoidAction>::Set as Monoid>::identity)
                    .take(min)
                    .chain(iter),
            )
            .into_boxed_slice();

            let lazy = Vec::from_iter(
                std::iter::repeat_with(<<Action as MonoidAction>::Map as Monoid>::identity)
                    .take(min),
            )
            .into_boxed_slice();

            let segment_size = <Action as MonoidAction>::USE_SEGMENT_SIZE.then(|| {
                let mut segment_size =
                    Vec::from_iter(std::iter::repeat_n(0, min).chain(std::iter::repeat_n(1, min)));
                for i in (1..min).rev() {
                    segment_size[i] = segment_size[i << 1] + segment_size[(i << 1) | 1]
                }
                segment_size.truncate(min);

                segment_size.into_boxed_slice()
            });

            let mut lst = Self {
                data,
                lazy,
                segment_size,
            };
            lst.recalculate_all();
            lst
        } else {
            Vec::from_iter(iter).into()
        }
    }
}

impl<Action> Debug for LazySegmentTree<Action>
where
    <<Action as MonoidAction>::Set as Monoid>::Set: Debug,
    <<Action as MonoidAction>::Map as Monoid>::Set: Debug,
    Action: MonoidAction,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LazySegmentTree")
            .field("data", &self.data)
            .field("lazy", &self.lazy)
            .field("segment_size", &self.segment_size)
            .finish()
    }
}

impl<Action> Clone for LazySegmentTree<Action>
where
    <<Action as MonoidAction>::Set as Monoid>::Set: Clone,
    <<Action as MonoidAction>::Map as Monoid>::Set: Clone,
    Action: MonoidAction,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            lazy: self.lazy.clone(),
            segment_size: self.segment_size.clone(),
        }
    }
}
