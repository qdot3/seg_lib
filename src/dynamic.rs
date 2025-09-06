use std::{
    fmt::Debug,
    num::NonZeroUsize,
    ops::{Range, RangeBounds},
};

use crate::traits::Monoid;

/// A data structure that supports **range query point update** operations on large array.
// ANCHOR: definition
pub struct DynamicSegmentTree<Query>
where
    Query: Monoid,
{
    arena: Vec<Node<<Query as Monoid>::Set>>,
    range: Range<isize>,

    // save allocation cost
    reusable_stack: Vec<usize>,
}
// ANCHOR_END: definition

impl<Query> DynamicSegmentTree<Query>
where
    Query: Monoid,
{
    #[doc = include_str!("../doc/dyn_new.org")]
    /// # Time complexity
    ///
    /// *O*(1)
    ///
    /// # Example
    ///
    /// ```rust
    /// use seg_lib::{DynamicSegmentTree, ops::BitOr};
    ///
    /// let mut dst = DynamicSegmentTree::<BitOr<u32>>::new(-100..100).unwrap();
    /// ```
    #[inline]
    pub fn new(range: Range<isize>) -> Option<Self> {
        if range.is_empty() {
            None
        } else {
            Some(Self {
                arena: Vec::new(),
                range,
                reusable_stack: Vec::new(),
            })
        }
    }

    #[doc = include_str!("../doc/dyn_with_capacity.org")]
    /// # Time complexity
    ///
    /// *O*(1)
    ///
    /// # Example
    ///
    /// ```rust
    /// use seg_lib::{DynamicSegmentTree, ops::Add};
    ///
    /// let num_query = 10_000;
    /// // avoid reallocation
    /// let mut dst = DynamicSegmentTree::<Add<i32>>::with_capacity(-100..100, num_query).unwrap();
    /// ```
    #[inline]
    // ANCHOR: with_capacity
    pub fn with_capacity(range: Range<isize>, q: usize) -> Option<Self> {
        if range.is_empty() {
            None
        } else {
            // never panic: `range.len()` is always larger than 0
            let height = range.len().ilog2() as usize + 1;
            Some(Self {
                arena: Vec::with_capacity(q * height),
                reusable_stack: Vec::with_capacity(height * 4),
                range,
            })
        }
    }
    // ANCHOR_END: with_capacity

    /// Returns the number of elements.
    ///
    /// # Time complexity
    ///
    /// *O*(1)
    ///
    /// # Example
    ///
    /// ```rust
    /// use seg_lib::{DynamicSegmentTree, ops::GCD};
    ///
    /// let range = -100..100;
    /// let mut dst = DynamicSegmentTree::<GCD<i32>>::new(range.clone()).unwrap();
    /// assert_eq!(dst.len(), range.len());
    ///
    /// // no effect
    /// dst.point_update(0, 999);
    /// assert_eq!(dst.len(), range.len());
    /// ```
    #[inline]
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.range.len()
    }

    #[doc = include_str!("../doc/point_update.org")]
    /// # Time complexity
    ///
    /// *O*(log *N*)
    ///
    /// # Example
    ///
    /// ```rust
    /// use seg_lib::{DynamicSegmentTree, ops::Mul};
    ///
    /// let mut dst = DynamicSegmentTree::<Mul<i32>>::new(-100..100).unwrap();
    /// assert_eq!(dst.point_query(0), 1);
    ///
    /// dst.point_update(0, 9);
    /// assert_eq!(dst.point_query(0), 9);
    /// ```
    pub fn point_update(&mut self, mut i: isize, mut element: <Query as Monoid>::Set) {
        assert!(self.range.contains(&i),);

        if self.arena.is_empty() {
            self.arena.push(Node::new(i, element));
            return;
        }

        // points to parent node
        let mut p_ptr = 0;
        let Range { mut start, mut end } = self.range;
        loop {
            // for recalculating combined values
            self.reusable_stack.push(p_ptr);

            if self.arena[p_ptr].index == i {
                self.arena[p_ptr].element = element;
                break;
            }

            macro_rules! descend_or_grow {
                ( $index_constraint:expr, $get_child_ptr:ident, $update_range_bounds:expr, $set_child:ident ) => {
                    if !($index_constraint) {
                        std::mem::swap(&mut i, &mut self.arena[p_ptr].index);
                        std::mem::swap(&mut element, &mut self.arena[p_ptr].element);
                    }

                    if let Some(c_ptr) = self.arena[p_ptr].$get_child_ptr() {
                        // descend
                        p_ptr = c_ptr;
                        $update_range_bounds;
                        continue;
                    } else {
                        // or grow
                        let n = self.arena.len();
                        self.arena[p_ptr].$set_child(n);

                        self.arena.push(Node::new(i, element));
                        break;
                    }
                };
            }

            let mid = start.midpoint(end);
            if i < mid {
                descend_or_grow!(
                    i < self.arena[p_ptr].index, // i_l < i_p
                    get_left_ptr,
                    end = mid, // [start, end) -> [start, mid)
                    set_left_ptr
                );
            } else {
                descend_or_grow!(
                    i > self.arena[p_ptr].index, // i_r > i_p
                    get_right_ptr,
                    start = mid, // [start, end) -> [mid, end)
                    set_right_ptr
                );
            }
        }

        // recalculate `combined` value in bottom-to-top order
        while let Some(ptr) = self.reusable_stack.pop() {
            let mut combined = <Query as Monoid>::identity();

            if let Some(l_ptr) = self.arena[ptr].get_left_ptr() {
                combined = <Query as Monoid>::combine(&combined, self.arena[l_ptr].get_combined())
            }
            combined = <Query as Monoid>::combine(&combined, self.arena[ptr].get_element());
            if let Some(r_ptr) = self.arena[ptr].get_right_ptr() {
                combined = <Query as Monoid>::combine(&combined, self.arena[r_ptr].get_combined())
            }

            self.arena[ptr].set_combined(combined);
        }
    }

    #[doc = include_str!("../doc/range_query.org")]
    /// # Time complexity
    ///
    /// *O*(log *N*)
    ///
    /// # Example
    ///
    /// ```rust
    /// use seg_lib::{DynamicSegmentTree, ops::LCM};
    ///
    /// let mut dst = DynamicSegmentTree::<LCM<i32>>::new(-100..100).unwrap();
    ///
    /// dst.point_update(-50, 9);
    /// dst.point_update(-40, 3);
    /// dst.point_update(-30, 7);
    ///
    /// assert_eq!(dst.range_query(..), 9 * 7);
    /// assert_eq!(dst.range_query(0..), 1);
    /// assert_eq!(dst.range_query(..=-40), 9);
    /// ```
    pub fn range_query<R>(&mut self, range: R) -> <Query as Monoid>::Set
    where
        R: RangeBounds<isize>,
    {
        let Range { start, end } = self.range;
        let l = match range.start_bound() {
            std::ops::Bound::Included(l) => *l,
            std::ops::Bound::Excluded(l) => l + 1,
            std::ops::Bound::Unbounded => start,
        };
        let r = match range.end_bound() {
            std::ops::Bound::Included(r) => r + 1,
            std::ops::Bound::Excluded(r) => *r,
            std::ops::Bound::Unbounded => end,
        };

        if l >= r || self.arena.is_empty() {
            return <Query as Monoid>::identity();
        }

        // Step 1: descend until the given `range` is within only one child.
        let mut p_ptr = 0;
        let [mut start, mut end] = [start, end];
        // The capacity of `Vec<T>`does NOT exceeds `isize::MAX`.
        // See [`Vec::with_capacity()`], [`Vec::push()`].
        assert!(self.arena.len() <= isize::MAX as usize);
        assert_eq!(isize::MAX as usize, usize::MAX >> 1);
        while let Some(node) = self.arena.get(p_ptr) {
            let mid = start.midpoint(end);
            if l >= mid
                && let Some(r_ptr) = node.get_right_ptr()
            {
                if (l..r).contains(&node.index) {
                    self.reusable_stack.push(p_ptr);
                }
                p_ptr = r_ptr;
                start = mid;
            } else if r <= mid
                && let Some(l_ptr) = node.get_left_ptr()
            {
                if (l..r).contains(&node.index) {
                    self.reusable_stack.push(!p_ptr);
                }
                p_ptr = l_ptr;
                end = mid;
            } else {
                break;
            }
        }

        // Step 2
        let p_ptr = p_ptr;
        let [start, end] = [start, end];
        let mid = start.midpoint(end);

        // (a) l <= i < mid
        let mut res = <Query as Monoid>::identity();
        if let Some(mut p_ptr) = self.arena[p_ptr].get_left_ptr() {
            let [mut start, mut end] = [start, mid];
            while let Some(node) = self.arena.get(p_ptr) {
                if l <= start && end <= r {
                    res = <Query as Monoid>::combine(node.get_combined(), &res);
                    break;
                }

                let mid = start.midpoint(end);
                if l < mid {
                    if let Some(r_ptr) = node.get_right_ptr() {
                        res = <Query as Monoid>::combine(self.arena[r_ptr].get_combined(), &res)
                    }
                    if (l..r).contains(&node.index) {
                        res = <Query as Monoid>::combine(node.get_element(), &res)
                    }
                    if let Some(l_ptr) = node.get_left_ptr() {
                        p_ptr = l_ptr;
                        end = mid
                    } else {
                        break;
                    }
                } else {
                    if (l..r).contains(&node.index) {
                        self.reusable_stack.push(p_ptr);
                    }
                    if let Some(r_ptr) = node.get_right_ptr() {
                        p_ptr = r_ptr;
                        start = mid
                    } else {
                        break;
                    }
                }
            }
        }

        // (b) self
        if (l..r).contains(&self.arena[p_ptr].index) {
            res = <Query as Monoid>::combine(&res, self.arena[p_ptr].get_element());
        }

        // (c) mid <= i < r
        if let Some(mut p_ptr) = self.arena[p_ptr].get_right_ptr() {
            let [mut start, mut end] = [mid, end];
            while let Some(node) = self.arena.get(p_ptr) {
                if l <= start && end <= r {
                    res = <Query as Monoid>::combine(&res, node.get_combined());
                    break;
                }

                let mid = start.midpoint(end);
                if r > mid {
                    if let Some(l_ptr) = node.get_left_ptr() {
                        res = <Query as Monoid>::combine(&res, self.arena[l_ptr].get_combined());
                    }
                    if (l..r).contains(&node.index) {
                        res = <Query as Monoid>::combine(&res, node.get_element())
                    }
                    if let Some(r_ptr) = node.get_right_ptr() {
                        p_ptr = r_ptr;
                        start = mid;
                    } else {
                        break;
                    }
                } else {
                    if (l..r).contains(&node.index) {
                        self.reusable_stack.push(!p_ptr);
                    }
                    if let Some(l_ptr) = node.get_left_ptr() {
                        p_ptr = l_ptr;
                        end = mid;
                    } else {
                        break;
                    }
                }
            }
        }

        // Step 3
        // ANCHOR: reusable_stack
        while let Some(ptr) = self.reusable_stack.pop() {
            const MSB: usize = 1_usize.rotate_right(1);
            res = if ptr & MSB == 0 {
                <Query as Monoid>::combine(self.arena[ptr].get_element(), &res)
            } else {
                <Query as Monoid>::combine(&res, self.arena[!ptr].get_element())
            }
        }
        // ANCHOR_END: reusable_stack

        res
    }
}

impl<Query> DynamicSegmentTree<Query>
where
    Query: Monoid,
    <Query as Monoid>::Set: Clone,
{
    /// Answers query for i-th element.
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    ///
    /// # Example
    ///
    /// ```rust
    /// use seg_lib::{DynamicSegmentTree, ops::BitAnd};
    ///
    /// let mut dst = DynamicSegmentTree::<BitAnd<u32>>::new(-100..100).unwrap();
    ///
    /// dst.point_update(-50, 9);
    /// dst.point_update(-40, 3);
    /// dst.point_update(-30, 7);
    ///
    /// assert_eq!(dst.range_query(..), 9 & 3 & 7);
    /// assert_eq!(dst.range_query(0..), !0);
    /// assert_eq!(dst.range_query(..=-40), 9 & 3);
    /// ```
    pub fn point_query(&self, i: isize) -> <Query as Monoid>::Set {
        if self.range.contains(&i) && !self.arena.is_empty() {
            let Range { mut start, mut end } = self.range;

            let mut p_ptr = 0;
            while let Some(node) = self.arena.get(p_ptr) {
                if node.index == i {
                    return node.get_element().clone();
                }

                let mid = start.midpoint(end);
                if i < mid
                    && let Some(l_ptr) = node.get_left_ptr()
                {
                    p_ptr = l_ptr;
                    end = mid;
                } else if i >= mid
                    && let Some(r_ptr) = node.get_right_ptr()
                {
                    p_ptr = r_ptr;
                    start = mid;
                } else {
                    break;
                }
            }
        }

        <Query as Monoid>::identity()
    }
}

impl<Query> Debug for DynamicSegmentTree<Query>
where
    Query: Monoid,
    <Query as Monoid>::Set: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynamicSegmentTree")
            .field("data", &self.arena)
            .field("range", &self.range)
            .field("reusable_stack", &self.reusable_stack)
            .finish()
    }
}

impl<Query> Clone for DynamicSegmentTree<Query>
where
    Query: Monoid,
    <Query as Monoid>::Set: Clone,
{
    fn clone(&self) -> Self {
        Self {
            arena: self.arena.clone(),
            range: self.range.clone(),
            reusable_stack: self.reusable_stack.clone(),
        }
    }
}

#[derive(Debug, Clone)]
// ANCHOR: node
struct Node<T> {
    index: isize,
    element: T,
    /// may be `None` if `combined == element`, avoiding `clone()`
    combined: Option<T>,

    left_ptr: Option<NonZeroUsize>,
    right_ptr: Option<NonZeroUsize>,
}
// ANCHOR_END: node

impl<T> Node<T> {
    #[inline]
    fn new(index: isize, element: T) -> Self {
        Self {
            index,
            element,
            combined: None,
            left_ptr: None,
            right_ptr: None,
        }
    }

    #[inline]
    fn get_left_ptr(&self) -> Option<usize> {
        self.left_ptr.map(|i| i.get())
    }

    #[inline]
    fn get_right_ptr(&self) -> Option<usize> {
        self.right_ptr.map(|i| i.get())
    }

    /// Invalid `ptr` will be ignored.
    #[inline]
    fn set_left_ptr(&mut self, ptr: usize) {
        self.left_ptr = NonZeroUsize::new(ptr)
    }

    /// Invalid `ptr` will be ignored.
    #[inline]
    fn set_right_ptr(&mut self, ptr: usize) {
        self.right_ptr = NonZeroUsize::new(ptr)
    }

    #[inline]
    fn get_element(&self) -> &T {
        &self.element
    }

    #[inline]
    fn get_combined(&self) -> &T {
        if let Some(combined) = self.combined.as_ref() {
            combined
        } else {
            &self.element
        }
    }

    #[inline]
    fn set_combined(&mut self, combined: T) {
        self.combined = Some(combined);
    }
}
