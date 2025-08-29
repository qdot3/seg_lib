use std::{
    fmt::Debug,
    marker::PhantomData,
    num::NonZeroUsize,
    ops::{Range, RangeBounds},
};

use crate::traits::Monoid;

/// A data structure that supports **range query point update** operations on large array.
pub struct DynamicSegmentTree<Query>
where
    Query: Monoid,
{
    data: Vec<Node<<Query as Monoid>::Set>>,
    range: Range<isize>,

    // save allocation cost
    reusable_stack: Vec<usize>,

    // for debug
    query: PhantomData<Query>,
}

impl<Query> DynamicSegmentTree<Query>
where
    Query: Monoid,
{
    /// Creates a new instance initialized with [identity elements](crate::traits::Monoid::identity()).
    ///
    /// Returns [`None`] if the given range is empty.
    ///
    /// # Time complexity
    ///
    /// *O*(1)
    #[inline]
    #[must_use]
    pub fn new(range: Range<isize>) -> Option<Self> {
        if range.is_empty() {
            None
        } else {
            Some(Self {
                data: Vec::new(),
                range,
                reusable_stack: Vec::new(),
                query: PhantomData,
            })
        }
    }

    /// Creates a new instance initialized with [identity elements](crate::traits::Monoid::identity())
    /// with at least specified `capacity`.
    ///
    /// Returns [`None`] if the given range is empty.
    ///
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
    #[must_use]
    pub fn with_capacity(range: Range<isize>, capacity: usize) -> Option<Self> {
        if range.is_empty() {
            None
        } else {
            Some(Self {
                data: Vec::with_capacity(capacity),
                reusable_stack: Vec::with_capacity((range.len() | 1).ilog2() as usize * 2),
                range,
                query: PhantomData,
            })
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
    #[must_use]
    pub fn len(&self) -> usize {
        self.range.len()
    }

    /// Replaces i-th element with given `element`.
    ///
    /// # Time complexity
    ///
    /// *O*(log *Q*)
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
        if self.data.is_empty() {
            self.data.push(Node::new(i, element));
            return;
        }

        // points to parent node
        let mut p_ptr = 0;
        let Range { mut start, mut end } = self.range;
        loop {
            // for recalculating `data`
            self.reusable_stack.push(p_ptr);

            if self.data[p_ptr].index == i {
                self.data[p_ptr].element = element;
                break;
            }

            let mid = start.midpoint(end);
            if i < mid {
                // i_l < i_p < i_r
                if i > self.data[p_ptr].index {
                    std::mem::swap(&mut i, &mut self.data[p_ptr].index);
                    std::mem::swap(&mut element, &mut self.data[p_ptr].element);
                }

                if let Some(l_ptr) = self.data[p_ptr].get_left_ptr() {
                    p_ptr = l_ptr;
                    end = mid;
                    continue;
                } else {
                    let n = self.data.len();
                    self.data[p_ptr].set_left_ptr(n);

                    self.data.push(Node::new(i, element));
                    break;
                }
            } else {
                // i_l < i_p < i_r
                if i < self.data[p_ptr].index {
                    std::mem::swap(&mut i, &mut self.data[p_ptr].index);
                    std::mem::swap(&mut element, &mut self.data[p_ptr].element);
                }

                if let Some(r_ptr) = self.data[p_ptr].get_right_ptr() {
                    p_ptr = r_ptr;
                    start = mid;
                    continue;
                } else {
                    let n = self.data.len();
                    self.data[p_ptr].set_right_ptr(n);

                    self.data.push(Node::new(i, element));
                    break;
                }
            }
        }

        // recalculate data in bottom-to-top order
        while let Some(ptr) = self.reusable_stack.pop() {
            let mut combined = <Query as Monoid>::identity();

            if let Some(l_ptr) = self.data[ptr].get_left_ptr() {
                combined = <Query as Monoid>::combine(&combined, self.data[l_ptr].get_combined())
            }
            combined = <Query as Monoid>::combine(&combined, self.data[ptr].get_element());
            if let Some(r_ptr) = self.data[ptr].get_right_ptr() {
                combined = <Query as Monoid>::combine(&combined, self.data[r_ptr].get_combined())
            }

            self.data[ptr].set_combined(combined);
        }
    }

    /// Answers query for the given range.
    ///
    /// If the given range is empty, returns [the identity element](crate::traits::Monoid::identity()).
    ///
    /// # Time complexity
    ///
    /// *O*(log *Q*)
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
    #[must_use]
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

        if l >= r || self.data.is_empty() {
            return <Query as Monoid>::identity();
        }

        // Step 1: go down until the given `range` is within only one child.
        let mut p_ptr = 0;
        let [mut start, mut end] = [start, end];
        // The capacity of `Vec<T>`does NOT exceeds `isize::MAX`.
        // See [`Vec::with_capacity()`], [`Vec::push()`].
        assert!(self.data.len() <= isize::MAX as usize);
        assert_eq!(isize::MAX as usize, usize::MAX >> 1);
        while let Some(node) = self.data.get(p_ptr) {
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
        if let Some(mut p_ptr) = self.data[p_ptr].get_left_ptr() {
            let [mut start, mut end] = [start, mid];
            while let Some(node) = self.data.get(p_ptr) {
                if l <= start && end <= r {
                    res = <Query as Monoid>::combine(node.get_combined(), &res);
                    break;
                }

                let mid = start.midpoint(end);
                if l < mid {
                    if let Some(r_ptr) = node.get_right_ptr() {
                        res = <Query as Monoid>::combine(self.data[r_ptr].get_combined(), &res)
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
        if (l..r).contains(&self.data[p_ptr].index) {
            res = <Query as Monoid>::combine(&res, self.data[p_ptr].get_element());
        }

        // (c) mid <= i < r
        if let Some(mut p_ptr) = self.data[p_ptr].get_right_ptr() {
            let [mut start, mut end] = [mid, end];
            while let Some(node) = self.data.get(p_ptr) {
                if l <= start && end <= r {
                    res = <Query as Monoid>::combine(&res, node.get_combined());
                    break;
                }

                let mid = start.midpoint(end);
                if r > mid {
                    if let Some(l_ptr) = node.get_left_ptr() {
                        res = <Query as Monoid>::combine(&res, self.data[l_ptr].get_combined());
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
                        end = mid
                    } else {
                        break;
                    }
                }
            }
        }

        // Step 3
        while let Some(ptr) = self.reusable_stack.pop() {
            res = if ptr <= usize::MAX >> 1 {
                <Query as Monoid>::combine(self.data[ptr].get_element(), &res)
            } else {
                <Query as Monoid>::combine(&res, self.data[!ptr].get_element())
            }
        }

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
    /// *O*(*Q*)
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
    /// assert_eq!(dst.range_query(0..), 1);
    /// assert_eq!(dst.range_query(..=-40), 9 & 3);
    /// ```
    pub fn point_query(&self, i: isize) -> <Query as Monoid>::Set {
        if self.range.contains(&i) && !self.data.is_empty() {
            let Range { mut start, mut end } = self.range;

            let mut p_ptr = 0;
            while let Some(node) = self.data.get(p_ptr) {
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
            .field("data", &self.data)
            .field("range", &self.range)
            .field("reusable_stack", &self.reusable_stack)
            .field("query", &self.query)
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
            data: self.data.clone(),
            range: self.range.clone(),
            reusable_stack: self.reusable_stack.clone(),
            query: self.query,
        }
    }
}

#[derive(Debug, Clone)]
struct Node<T> {
    index: isize,
    element: T,
    /// may be `None` if `combined == element`, avoiding `clone()`
    combined: Option<T>,

    left_ptr: Option<NonZeroUsize>,
    right_ptr: Option<NonZeroUsize>,
}

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
        let _ = self.combined.insert(combined);
    }
}
