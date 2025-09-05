use std::marker::PhantomData;

use seg_lib::{
    ops::{Add, AssignOr, Max}, DynamicLazySegmentTree, Monoid, MonoidAction
};

fn main() {
    let range = -1_000..4_000;
    let mut dlst =
        DynamicLazySegmentTree::<MaxQueryAssignOrAddUpdate<i32>>::new(range.clone()).unwrap();
    assert_eq!(dlst.len(), range.len());

    dlst.range_update(.., &AssignOr::Assign(Some(50)));
    assert_eq!(dlst.range_query(..), Some(50));

    dlst.range_update(..100, &AssignOr::Other(100));
    dlst.range_update(-100.., &AssignOr::Other(200));
    assert_eq!(dlst.range_query(..), Some(350));
    assert_eq!(dlst.range_query(..-500), Some(150));
    assert_eq!(dlst.range_query(500..), Some(250));
}

struct MaxQueryAssignOrAddUpdate<T>(PhantomData<T>);

impl MonoidAction for MaxQueryAssignOrAddUpdate<i32> {
    type Map = AssignOr<Add<i32>>;
    type Set = Max<i32>;

    const USE_SEGMENT_SIZE: bool = false;

    fn act(
        mapping: &<Self::Map as Monoid>::Set,
        element: &<Self::Set as Monoid>::Set,
        _: Option<usize>,
    ) -> <Self::Set as Monoid>::Set {
        match mapping {
            AssignOr::Assign(assign) => match assign {
                Some(new) => Some(*new),
                None => *element,
            },
            AssignOr::Other(add) => element.map(|element| element + add),
        }
    }
}
