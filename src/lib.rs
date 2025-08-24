//! `seg_lib` provides segment tree variants.

mod normal;
pub use normal::SegmentTree;

mod dual;
pub use dual::DualSegmentTree;

mod lazy;
pub use lazy::LazySegmentTree;

/// Predefined operations on segment tree variants.
///
/// # Custom provider
///
/// ```ignore
/// pub struct Custom<T>(PhantomData<T>);
///
/// impl<T> Monoid for Custom<T> { /* ---- */ }
/// ```
///
/// # TODO
///
/// If `negative_trait_impl` is stabilized, rewrite trait bounds as follows:
///
/// ```ignore
/// impl Monoid for Add<T>
/// where
///     T: !Copy
///     for<'a> &'a T: std::ops::Add<Output =T>
/// { /* methods */ }
///
/// impl Monoid for Add<T>
/// where
///     T: Copy + std::ops::Add<Output = T>
/// { /* methods */ }
/// ```
pub mod provider {
    mod add;
    pub use add::Add;

    mod max;
    pub use max::Max;

    mod assign;
    pub use assign::Assign;

    mod affine;
    pub use affine::Affine;
}

pub mod traits;
