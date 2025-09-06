use std::{
    fmt::Debug,
    num::NonZeroUsize,
    ops::{Range, RangeBounds},
};

use crate::traits::{Monoid, MonoidAction};

/// A data structure that supports **range query range update** operations on large array.
///
/// # Example
///
/// ```rust
#[doc = include_str!("../examples/ex_dynamic_lazy.rs")]
/// ```
pub struct DynamicLazySegmentTree<Action>
where
    Action: MonoidAction,
{
    arena: Vec<Node<<Action as MonoidAction>::Set, <Action as MonoidAction>::Map>>,
    range: Range<isize>,

    // save allocation cost
    reusable_buf: Vec<(usize, Range<isize>)>,
}

impl<Action> DynamicLazySegmentTree<Action>
where
    Action: MonoidAction,
{
    /// Creates a new instance initialized with `n` [identity elements](crate::traits::Monoid::identity()).
    ///
    /// # Time complexity
    ///
    /// *O*(1)
    #[inline]
    pub fn new(range: Range<isize>) -> Option<Self> {
        if range.is_empty() {
            None
        } else {
            Some(Self {
                arena: vec![Node::new()],
                reusable_buf: Vec::with_capacity((range.len().ilog2() as usize + 1) << 2),
                range,
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
    #[inline]
    pub fn with_capacity(range: Range<isize>, capacity: usize) -> Option<Self> {
        if range.is_empty() {
            None
        } else {
            // never panic
            let height = range.len().ilog2() as usize + 1;
            Some(Self {
                arena: {
                    let mut arena = Vec::with_capacity(capacity * height);
                    arena.push(Node::new());
                    arena
                },
                range,
                reusable_buf: Vec::with_capacity(height << 2),
            })
        }
    }

    /// Returns the number of elements.
    ///
    /// # Time complexity
    ///
    /// *O*(1)
    #[inline]
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.range.len()
    }

    /// Returns [L, r)
    #[inline]
    fn translate_range<R>(&self, range: R) -> [isize; 2]
    where
        R: RangeBounds<isize>,
    {
        let l = match range.start_bound() {
            std::ops::Bound::Included(l) => *l,
            std::ops::Bound::Excluded(l) => l + 1,
            std::ops::Bound::Unbounded => self.range.start,
        };
        let r = match range.end_bound() {
            std::ops::Bound::Included(r) => r + 1,
            std::ops::Bound::Excluded(r) => *r,
            std::ops::Bound::Unbounded => self.range.end,
        };

        [l, r]
    }

    fn push_map(
        &mut self,
        ptr: usize,
        range: Range<isize>,
        update: &<<Action as MonoidAction>::Map as Monoid>::Set,
    ) {
        assert!(!range.is_empty(), "invalid node");
        let node = &mut self.arena[ptr];

        node.element = <Action as MonoidAction>::act(update, &node.element, Some(range.len()));
        node.update = <<Action as MonoidAction>::Map as Monoid>::combine(&node.update, update)
    }

    fn propagate_at(&mut self, ptr: usize, range: Range<isize>) {
        assert!(
            range.len() >= 2,
            "no child error: the node `ptr` points to should have two children"
        );

        let update = std::mem::replace(
            &mut self.arena[ptr].update,
            <<Action as MonoidAction>::Map as Monoid>::identity(),
        );

        let Range { start, end } = range;
        let mid = start.midpoint(end);

        {
            let l_ptr = if let Some(l_ptr) = self.arena[ptr].get_left_ptr() {
                l_ptr
            } else {
                let l_ptr = self.arena.len();
                self.arena.push(Node::new());
                self.arena[ptr].set_left_ptr(l_ptr);
                l_ptr
            };
            self.push_map(l_ptr, start..mid, &update);
        }
        {
            let r_ptr = if let Some(r_ptr) = self.arena[ptr].get_right_ptr() {
                r_ptr
            } else {
                let r_ptr = self.arena.len();
                self.arena.push(Node::new());
                self.arena[ptr].set_right_ptr(r_ptr);
                r_ptr
            };
            self.push_map(r_ptr, mid..end, &update);
        }
    }

    /// Updates elements in the range using [predefined binary operation](crate::traits::Monoid::combine()).
    /// More precisely, `a[i] <- update · a[i], i ∈ range`
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    pub fn range_update<R>(
        &mut self,
        range: R,
        update: &<<Action as MonoidAction>::Map as Monoid>::Set,
    ) where
        R: RangeBounds<isize>,
    {
        let [l, r] = self.translate_range(range);
        if l >= r {
            return;
        }

        self.reusable_buf.push((0, self.range.clone()));
        let mut i = 0;
        while let Some((ptr, range)) = self.reusable_buf.get(i).cloned() {
            let Range { start, end } = range;

            if l <= start && end <= r {
                // push given update
                self.push_map(ptr, range.clone(), update);
                if range.len() >> 1 != 0 {
                    self.propagate_at(ptr, range);
                }
            } else {
                // lazy propagation in top-to-bottom order
                self.propagate_at(ptr, range);

                let mid = start.midpoint(end);
                if l < mid {
                    self.reusable_buf
                        .push((self.arena[ptr].get_left_ptr().unwrap(), start..mid));
                }
                if r > mid {
                    self.reusable_buf
                        .push((self.arena[ptr].get_right_ptr().unwrap(), mid..end));
                }
            }

            i += 1
        }

        // recalculate in bottom-to-top order
        while let Some((ptr, _)) = self.reusable_buf.pop() {
            assert!(
                self.arena[ptr].get_left_ptr().is_some()
                    == self.arena[ptr].get_right_ptr().is_some()
            );
            if let Some(l_ptr) = self.arena[ptr].get_left_ptr()
                && let Some(r_ptr) = self.arena[ptr].get_right_ptr()
            {
                self.arena[ptr].element = <<Action as MonoidAction>::Set as Monoid>::combine(
                    &self.arena[l_ptr].element,
                    &self.arena[r_ptr].element,
                )
            }
        }
    }

    /// Answers query for the given range.
    ///
    /// If the given range is empty, returns [the identity element](crate::traits::Monoid::identity()).
    ///
    /// # Time complexity
    ///
    /// *O*(log *N*)
    pub fn range_query<R>(&mut self, range: R) -> <<Action as MonoidAction>::Set as Monoid>::Set
    where
        R: RangeBounds<isize>,
    {
        let [l, r] = self.translate_range(range);
        if l >= r {
            return <<Action as MonoidAction>::Set as Monoid>::identity();
        }

        let self_mid = self.range.start.midpoint(self.range.end);
        let mut res = <<Action as MonoidAction>::Set as Monoid>::identity();

        self.reusable_buf.push((0, self.range.clone()));
        let mut i = 0;
        while let Some((ptr, range)) = self.reusable_buf.get(i).cloned() {
            const MSB: usize = 1_usize.rotate_right(1);
            let Range { start, end } = range;

            if l <= start && end <= r {
                // calculate answer
                if ptr & MSB == 0 {
                    res = <<Action as MonoidAction>::Set as Monoid>::combine(
                        &self.arena[ptr].element,
                        &res,
                    )
                } else {
                    res = <<Action as MonoidAction>::Set as Monoid>::combine(
                        &res,
                        &self.arena[!ptr].element,
                    )
                }
            } else {
                // lazy propagation in top-to-bottom order
                let ptr = if ptr & MSB == 0 { ptr } else { !ptr };
                self.propagate_at(ptr, range);

                let mid = start.midpoint(end);
                let is_left_size = mid < self_mid;
                let mut pushed = 0;
                if l < mid {
                    let l_ptr = self.arena[ptr].get_left_ptr().unwrap();
                    self.reusable_buf
                        .push((if is_left_size { l_ptr } else { !l_ptr }, start..mid));
                    pushed += 1;
                }
                if r > mid {
                    let r_ptr = self.arena[ptr].get_right_ptr().unwrap();
                    self.reusable_buf
                        .push((if is_left_size { r_ptr } else { !r_ptr }, mid..end));
                    pushed += 1
                }
                if pushed == 2 && is_left_size {
                    let n = self.reusable_buf.len();
                    self.reusable_buf.swap(n - 1, n - 2);
                }
            }

            i += 1
        }
        self.reusable_buf.clear();

        res
    }
}

impl<Action> Debug for DynamicLazySegmentTree<Action>
where
    <<Action as MonoidAction>::Set as Monoid>::Set: Debug,
    <<Action as MonoidAction>::Map as Monoid>::Set: Debug,
    Action: MonoidAction,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynamicLazySegmentTree")
            .field("arena", &self.arena)
            .field("range", &self.range)
            .field("reusable_buf", &self.reusable_buf)
            .finish()
    }
}

impl<Action> Clone for DynamicLazySegmentTree<Action>
where
    <<Action as MonoidAction>::Set as Monoid>::Set: Clone,
    <<Action as MonoidAction>::Map as Monoid>::Set: Clone,
    Action: MonoidAction,
{
    fn clone(&self) -> Self {
        Self {
            arena: self.arena.clone(),
            range: self.range.clone(),
            reusable_buf: self.reusable_buf.clone(),
        }
    }
}

struct Node<Query, Update>
where
    Query: Monoid,
    Update: Monoid,
{
    element: <Query as Monoid>::Set,
    update: <Update as Monoid>::Set,

    // index on arena
    left_ptr: Option<NonZeroUsize>,
    right_ptr: Option<NonZeroUsize>,
}

impl<Query, Update> Node<Query, Update>
where
    Query: Monoid,
    Update: Monoid,
{
    #[inline]
    fn new() -> Self {
        Self {
            element: <Query as Monoid>::identity(),
            update: <Update as Monoid>::identity(),
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
}

impl<Query, Update> Debug for Node<Query, Update>
where
    Query: Monoid,
    <Query as Monoid>::Set: Debug,
    Update: Monoid,
    <Update as Monoid>::Set: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("element", &self.element)
            .field("update", &self.update)
            .field("left_ptr", &self.left_ptr)
            .field("right_ptr", &self.right_ptr)
            .finish()
    }
}

impl<Query, Update> Clone for Node<Query, Update>
where
    Query: Monoid,
    <Query as Monoid>::Set: Clone,
    Update: Monoid,
    <Update as Monoid>::Set: Clone,
{
    fn clone(&self) -> Self {
        Self {
            element: self.element.clone(),
            update: self.update.clone(),
            left_ptr: self.left_ptr,
            right_ptr: self.right_ptr,
        }
    }
}
