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
    /// # Caution
    ///
    /// If the operation is **not** commutative, the position of the arguments matters.
    fn combine(lhs_or_prev: &Self::Set, rhs_or_new: &Self::Set) -> Self::Set;
}

/// A **monoid action** is a function `*: M x S -> S` of a monoid `M` on a set `S`.
///
/// # Lows
///
/// ```text
/// (1) (Map, ·, e) is a monoid
/// (2) (f · g) * a = f * (g * a)    ∀ f, g ∈ M, ∀ a ∈ S
/// (3) e * a = a                    ∃   e  ∈ M, ∀ a ∈ S
/// ```
///
/// See [Monoid] for reference.

pub trait MonoidAction {
    type Map: Monoid;
    type Set: Monoid;

    /// Set [`true`] to use the segment size in [`Self::act()`]
    const USE_SEGMENT_SIZE: bool;

    /// # Low
    ///
    /// ```text
    /// Σ_i f(a_i) = f(Σ_i a_i)
    /// ```
    ///
    /// # Size dependency
    ///
    /// You can access segment size if you want.
    /// This is equivalent to attaching the segment size information to [`Self::Set`] as follows:
    ///
    /// ```text
    /// Σ_{l <= i < r} f(a_i, 1) = f(Σ_{l <= i < r} a_i, r-l)
    /// ```
    fn act(
        mapping: &<Self::Map as Monoid>::Set,
        element: &<Self::Set as Monoid>::Set,
        size: Option<usize>,
    ) -> <Self::Set as Monoid>::Set;
}
