use seg_lib::{AssignSegmentTree, ops::BitXor};

/// Demonstrates how to use an [`AssignSegmentTree`] for:
/// - range XOR queries
/// - range assign updates
fn main() {
    // Initialize a segment tree with values 0..100
    let mut seg = AssignSegmentTree::<BitXor<u32>>::from_iter(0..100);
    assert_eq!(seg.len(), 100);
    assert_eq!(Vec::from_iter(seg.iter().copied()), Vec::from_iter(0..100));

    // Assign values to a range
    seg.range_assign(..50, 100);
    assert!(seg.iter().take(50).all(|e| *e == 100));

    // Query XOR over ranges
    assert_eq!(seg.range_query(..50), 0); // 50 is even, so XOR cancels out
    assert_eq!(seg.range_query(50..), (50..100).fold(0, |res, e| res ^ e));

    // Assign and query a single element
    seg.point_assign(50, !0);
    assert_eq!(seg.point_query(50), &!0);
}
