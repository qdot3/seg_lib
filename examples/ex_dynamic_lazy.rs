use seg_lib::{DynamicLazySegmentTree, acts::MaxQueryAddOrAssignUpdate, ops::AssignOr};

/// Demonstrates how to use a [`DynamicLazySegmentTree`] for:
/// - range maximum queries (RMQ)
/// - range assignment and addition updates
fn main() {
    // Create a dynamic lazy segment tree over `-1_000..4_000`,
    // initialized with `None` (the identity element for RMQ).
    let range = -1_000..4_000;
    let mut dlst =
        DynamicLazySegmentTree::<MaxQueryAddOrAssignUpdate<i32>>::new(range.clone()).unwrap();
    assert_eq!(dlst.len(), range.len());
    assert_eq!(dlst.range_query(..), None);

    // Reinitialize with `50`
    dlst.range_update(.., &AssignOr::Assign(Some(50)));
    assert_eq!(dlst.range_query(..), Some(50));

    // Apply range additions
    dlst.range_update(..100, &AssignOr::Other(100));
    dlst.range_update(-100.., &AssignOr::Other(200));

    // Query maximum values
    assert_eq!(dlst.range_query(..), Some(350));
    assert_eq!(dlst.range_query(..=-500), Some(150));
    assert_eq!(dlst.range_query(500..), Some(250));
}
