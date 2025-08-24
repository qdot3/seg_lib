/// A **monoid** is a set equipped with:
///
/// - An associative binary operation
/// - An identity element
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
    /// If the operation is **not** commutative, be careful of the order of elements.
    fn combine(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set;
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
pub trait MonoidWithAction {
    type Map;
    type Set;

    /// If [`Self::combine`] is commutative, some operations can be optimized.
    ///
    /// If unsure about the commutativity, use [`false`] for safety.
    ///
    /// # Commutative low
    ///
    /// ```text
    /// f · g = g · f    ∀ f, g ∈ Map
    /// ```
    const IS_COMMUTATIVE: bool;

    ///
    const USE_SEGMENT_SIZE: bool;

    /// Returns the identity mapping.
    fn identity() -> Self::Map;

    /// Combines the two maps in chronological order.
    fn combine(prev: &mut Self::Map, new: &Self::Map);

    /// Acts the mapping on the element and returns the result.
    ///
    /// In a [`LazySegmentTree`](crate::lazy::LazySegmentTree), updates are applied simultaneously
    /// to `2^n` elements internally.
    /// If update operation depends on the size of the segment, set [`Self::USE_SEGMENT_SIZE`] to [`true`].
    fn act(mapping: &Self::Map, element: &Self::Set, size: Option<usize>) -> Self::Set;
}
