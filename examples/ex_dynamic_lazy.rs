use seg_lib::{DynamicLazySegmentTree, acts::MaxQueryAddOrAssignUpdate, ops::AssignOr};

/// Demonstrates how to use a [`DynamicLazySegmentTree`] for:
/// - range maximum queries (RMQ)
/// - range assignment and addition updates
fn main() {
    // Initialize a dynamic lazy segment tree over -1_000..4_000 with `None`
    let range = -1_000..4_000;
    let mut seg =
        DynamicLazySegmentTree::<MaxQueryAddOrAssignUpdate<i32>>::new(range.clone()).unwrap();
    assert_eq!(seg.len(), range.len());
    assert_eq!(seg.range_query(..), None);

    // Assign 50 to all elements
    seg.range_update(.., &AssignOr::Assign(Some(50)));
    assert_eq!(seg.range_query(..), Some(50));

    // Apply additions
    seg.range_update(..100, &AssignOr::Other(100));
    seg.range_update(-100.., &AssignOr::Other(200));

    // Query maximum values
    assert_eq!(seg.range_query(..), Some(350));
    assert_eq!(seg.range_query(..=-500), Some(150));
    assert_eq!(seg.range_query(500..), Some(250));
}
