use std::{
    marker::PhantomData,
    num::NonZeroUsize,
    ops::{Range, RangeBounds},
};

use crate::traits::{Monoid, MonoidAction};

/// A data structure that supports **range query range update** operations on large array.
pub struct DynamicLazySegmentTree<Query, Update>
where
    Query: Monoid,
    Update: Monoid + MonoidAction<Map = Update, Set = Query>,
{
    arena: Vec<Node<Query, Update>>,
    range: Range<isize>,

    // save allocation cost
    reusable_stack: Vec<usize>,

    // for debug
    query: PhantomData<Query>,
    update: PhantomData<Update>,
}

impl<Query, Update> DynamicLazySegmentTree<Query, Update>
where
    Query: Monoid,
    <Query as Monoid>::Set: Clone,
    Update: Monoid + MonoidAction<Map = Update, Set = Query>,
    <Update as Monoid>::Set: Clone,
{
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

    pub fn range_update<R>(&mut self, range: R, update: <Update as Monoid>::Set)
    where
        R: RangeBounds<isize>,
    {
        let [l, r] = self.translate_range(range);

        let Range { mut start, mut end } = self.range;

        // lazy propagation in top-to-bottom order
        let mut p_ptr = 0;
        loop {
            let mid = start.midpoint(end);
            if l >= mid {
                // propagate
                let update = self.arena[p_ptr].take_update();
                if let Some(ptr) = self.arena[p_ptr].get_left_ptr() {
                    <Update as MonoidAction>::act(
                        &update,
                        &mut self.arena[ptr].element,
                        Some((mid - start) as usize),
                    );
                    self.arena[ptr].update =
                        <Update as Monoid>::combine(&self.arena[ptr].update, &update)
                } else {
                    let ptr = self.arena.len();
                    self.arena[p_ptr].set_left_ptr(ptr);

                    let mut element = <Query as Monoid>::identity();
                    <Update as MonoidAction>::act(
                        &update,
                        &mut element,
                        Some((mid - start) as usize),
                    );
                    self.arena.push(Node::new(element, update));
                }
            } else if r <= mid {
            } else {
            }
        }
    }
}

struct Node<Query, Update>
where
    Query: Monoid,
    Update: Monoid + MonoidAction<Map = Update, Set = Query>,
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
    Update: Monoid + MonoidAction<Map = Update, Set = Query>,
{
    #[inline]
    fn new(element: <Query as Monoid>::Set, update: <Update as Monoid>::Set) -> Self {
        Self {
            element,
            update,
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
    fn take_update(&mut self) -> <Update as Monoid>::Set {
        std::mem::replace(&mut self.update, <Update as Monoid>::identity())
    }
}
