//! Traits that abstracts the algebra that can be applied over a segment tree variants.

/// A **monoid** is a set equipped with an associative binary operation and an identity element.
///
/// # Lows
///
/// ```text
/// (1) a · (b · c) = (a · b) · c    ∀ a, b, c ∈ Set
/// (2) a · e = e · a = a            ∀ a ∈ Set, ∃ e ∈ Set
/// ```
///
/// where `·` is the binary operation and `e` is the identity element.
pub trait Monoid {
    type Set;

    /// If [`Self::combine`] is commutative, some operations can be optimized.
    ///
    /// If unsure about the commutativity, use [`false`] for safety.
    ///
    /// # Commutative low
    ///
    /// ```text
    /// a · b = b · a    ∀ a, b ∈ Set
    /// ```
    const IS_COMMUTATIVE: bool;

    /// Returns the identity element.
    fn identity() -> Self::Set;

    /// Combines the two elements and returns the result.
    ///
    /// # Warning
    ///
    /// If the operation is **not** commutative, the position of the arguments matters.
    fn combine(lhs_or_prev: &Self::Set, rhs_or_new: &Self::Set) -> Self::Set;
}

macro_rules! monoid_tuple_impl {
    ( $( ($ty_names:ident, $indexes:tt), )* ) => {
        impl<$( $ty_names, )*> Monoid for ($( $ty_names, )*)
        where
            $( $ty_names: Monoid, )*
        {
            type Set = ($( <$ty_names as Monoid>::Set, )*);

            const IS_COMMUTATIVE: bool = true $( & <$ty_names as Monoid>::IS_COMMUTATIVE )*;

            fn identity() -> Self::Set {
                ($( <$ty_names as Monoid>::identity(), )*)
            }

            fn combine(lhs_or_prev: &Self::Set, rhs_or_new: &Self::Set) -> Self::Set {
                ($( <$ty_names as Monoid>::combine(&lhs_or_prev.$indexes, &rhs_or_new.$indexes), )*)
            }
        }
    };
}

monoid_tuple_impl!((M0, 0), (M1, 1),);
monoid_tuple_impl!((M0, 0), (M1, 1), (M2, 2),);
monoid_tuple_impl!((M0, 0), (M1, 1), (M2, 2), (M3, 3),);

/// A **monoid action** is a function `*: M x S -> S` of a monoid `M` on a monoid `S`.
///
/// # Low
///
/// ```text
/// Π_i f(a_i) = f(Σ_i a_i)
/// ```
///
/// See [Monoid] for reference.
pub trait MonoidAction {
    type Map: Monoid;
    type Set: Monoid;

    /// Set [`true`] to use the segment size in [`Self::act()`]
    const USE_SEGMENT_SIZE: bool;

    /// Acts the mapping on the element and returns the result.
    ///
    /// # Size dependency
    ///
    /// You can access segment size if you want.
    /// This is equivalent to attaching the segment size information to [`Self::Set`] as follows:
    ///
    /// ```text
    /// Π_{l <= i < r} f(a_i, 1) = f(Σ_{l <= i < r} a_i, r-l)
    /// ```
    fn act(
        mapping: &<Self::Map as Monoid>::Set,
        element: &<Self::Set as Monoid>::Set,
        size: Option<usize>,
    ) -> <Self::Set as Monoid>::Set;
}

/// A function that behaves like a monoid action under well-defined conditions,
/// which frequently hold in practice.
///
/// # Low
///
/// ```text
/// Π_i f(a_i) = f(Σ_i a_i)    in most cases
/// Π_i f(a_i) ≠ f(Σ_i a_i)    in rare cases
/// ```
///
/// See [`MonoidAction`] for details.
pub trait QuasiMonoidAction {
    type Map: Monoid;
    type Set: Monoid;

    /// Set [`true`] to use the segment size in [`Self::try_act()`]
    const USE_SEGMENT_SIZE: bool;

    /// Acts the mapping on the element and returns the result.
    ///
    /// # Size dependency
    ///
    /// You can access segment size if you want.
    /// This is equivalent to attaching the segment size information to [`Self::Set`] as follows:
    ///
    /// ```text
    /// Π_{l <= i < r} f(a_i, 1) = f(Σ_{l <= i < r} a_i, r-l)
    /// ```
    fn try_act(
        mapping: &<Self::Map as Monoid>::Set,
        element: &<Self::Set as Monoid>::Set,
        size: Option<usize>,
    ) -> Result<<Self::Set as Monoid>::Set, ()>;
}
