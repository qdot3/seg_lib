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

impl<M1, M2> Monoid for (M1, M2)
where
    M1: Monoid,
    M2: Monoid,
{
    type Set = (<M1 as Monoid>::Set, <M2 as Monoid>::Set);

    const IS_COMMUTATIVE: bool = <M1 as Monoid>::IS_COMMUTATIVE & <M2 as Monoid>::IS_COMMUTATIVE;

    fn identity() -> Self::Set {
        (<M1 as Monoid>::identity(), <M2 as Monoid>::identity())
    }

    fn combine(lhs_or_prev: &Self::Set, rhs_or_new: &Self::Set) -> Self::Set {
        (
            <M1 as Monoid>::combine(&lhs_or_prev.0, &rhs_or_new.0),
            <M2 as Monoid>::combine(&lhs_or_prev.1, &rhs_or_new.1),
        )
    }
}

impl<M1, M2, M3> Monoid for (M1, M2, M3)
where
    M1: Monoid,
    M2: Monoid,
    M3: Monoid,
{
    type Set = (
        <M1 as Monoid>::Set,
        <M2 as Monoid>::Set,
        <M3 as Monoid>::Set,
    );

    const IS_COMMUTATIVE: bool = <M1 as Monoid>::IS_COMMUTATIVE
        & <M2 as Monoid>::IS_COMMUTATIVE
        & <M3 as Monoid>::IS_COMMUTATIVE;

    fn identity() -> Self::Set {
        (
            <M1 as Monoid>::identity(),
            <M2 as Monoid>::identity(),
            <M3 as Monoid>::identity(),
        )
    }

    fn combine(lhs_or_prev: &Self::Set, rhs_or_new: &Self::Set) -> Self::Set {
        (
            <M1 as Monoid>::combine(&lhs_or_prev.0, &rhs_or_new.0),
            <M2 as Monoid>::combine(&lhs_or_prev.1, &rhs_or_new.1),
            <M3 as Monoid>::combine(&lhs_or_prev.2, &rhs_or_new.2),
        )
    }
}

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
