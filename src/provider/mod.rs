//! Predefined operations on segment tree variants.
//!
//! # Custom provider
//!
//! ```ignore
//! pub struct Custom<T>(PhantomData<T>);
//!
//! impl<T> Monoid for Custom<T> { /* ---- */ }
//! ```
//!
//! # TODO
//!
//! If `negative_trait_impl` is stabilized, rewrite trait bounds as follows:
//!
//! ```ignore
//! impl Monoid for Add<T>
//! where
//!     T: !Copy
//!     for<'a> &'a T: std::ops::Add<Output =T>
//! { /* methods */ }
//!
//! impl Monoid for Add<T>
//! where
//!     T: Copy + std::ops::Add<Output = T>
//! { /* methods */ }
//! ```
mod add;
pub use add::Add;

mod mul;
pub use mul::Mul;

mod bit_and;
pub use bit_and::BitAnd;

mod bit_or;
pub use bit_or::BitOr;

mod bit_xor;
pub use bit_xor::BitXor;

mod max;
pub use max::Max;

mod min;
pub use min::Min;

mod assign;
pub use assign::Assign;

mod affine;
pub use affine::Affine;

mod gcd;

mod lcm;