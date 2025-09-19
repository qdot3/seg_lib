use std::{
    fmt::Debug,
    ops::{Range, RangeBounds},
};

/// Convert [`RangeBounds`] trait objects into [`Range`].
///
/// # Panics
///
/// Panics if the given range is out of bounds or
/// if exclusive starting point or inclusive end point is `usize::MAX`.
#[inline(always)]
pub(crate) fn convert_range<R>(given: R, outer: Range<usize>) -> Range<usize>
where
    R: RangeBounds<usize> + Debug,
{
    let start = match given.start_bound() {
        std::ops::Bound::Included(start) => *start,
        std::ops::Bound::Excluded(start) => start
            .checked_add(1)
            .expect("starting point of the given range is less than `usize::MAX`"),
        std::ops::Bound::Unbounded => outer.start,
    };
    let end = match given.end_bound() {
        std::ops::Bound::Excluded(end) => *end,
        std::ops::Bound::Included(end) => end
            .checked_add(1)
            .expect("end point of the given range is less than `usize::MAX`"),
        std::ops::Bound::Unbounded => outer.end,
    };

    assert!(
        outer.start <= start && end <= outer.end,
        "the given range should be within {outer:?}, but is {given:?}",
    );

    start..end
}

#[cfg(test)]
mod test_convert_range {
    use std::panic::catch_unwind;

    use crate::utility::convert_range;

    #[test]
    fn clamp_infinite_range() {
        assert_eq!(convert_range(.., 0..3), 0..3);
        assert_eq!(convert_range(1.., 0..3), 1..3);
        assert_eq!(convert_range(..2, 0..3), 0..2);
    }

    #[test]
    fn out_of_bounds() {
        assert!(catch_unwind(|| convert_range(0..3, 1..2)).is_err());
        assert!(catch_unwind(|| convert_range(0..2, 1..2)).is_err());
        assert!(catch_unwind(|| convert_range(1..3, 1..2)).is_err());
    }

    #[test]
    fn overflow() {
        assert!(catch_unwind(|| convert_range(..=!0, 0..!0)).is_err());
        assert!(catch_unwind(|| convert_range(.., 0..!0)).is_ok());
    }
}

/// Returns the smallest index of invalid nodes in segment tree variants.
///
/// - All its ancestor nodes are also invalid.
/// - 0th node is always invalid.
///
/// # Panics
///
/// Panics if `0` is passed.
#[deprecated = "useless"]
pub(crate) const fn min_invalid_index(len_plus_offset: usize) -> usize {
    let bottom_len = len_plus_offset - (len_plus_offset.next_power_of_two() >> 1);
    let valid_parent_num = bottom_len.trailing_zeros();
    (len_plus_offset - 1) >> (valid_parent_num + 1)
}

#[test]
fn test_min_invalid_index() {
    const MAX_N: usize = 1_000;

    fn calc_node_size(n: usize) -> Vec<usize> {
        let mut size = Vec::from_iter(std::iter::repeat_n(0, n).chain(std::iter::repeat_n(1, n)));
        for i in (0..n).rev() {
            size[i] = size[i * 2] + size[i * 2 + 1]
        }
        size
    }

    fn naive(size: &[usize]) -> usize {
        (0..size.len())
            .rev()
            .find(|&i| !size[i].is_power_of_two())
            .unwrap_or(0)
    }

    for n in 1..MAX_N {
        let size = calc_node_size(n);

        assert_eq!(min_invalid_index(n * 2), naive(&size))
    }
}
