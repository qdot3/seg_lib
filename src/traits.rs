/// Defines query on segment tree variants.
pub trait Query<T> {
    /// Returns identity element.
    fn identity() -> T;

    /// Combines two given elements.
    fn combine(lhs: &T, rhs: &T) -> T;
}

/// Defines update query on segment tree variants.
pub trait Update<M> {
    type Set;

    /// Returns identity mapping.
    fn identity() -> M;

    /// Composes given two mappings in chronological order.
    fn combine(prev: &M, next: &M) -> M;

    /// Lets a given `mapping` act on a given `element` and returns the result.
    fn update(mapping: &M, element: &Self::Set) -> Self::Set;
}

impl<P1, P2, T1, T2> Query<(T1, T2)> for (P1, P2)
where
    P1: Query<T1>,
    P2: Query<T2>,
{
    fn identity() -> (T1, T2) {
        (<P1 as Query<T1>>::identity(), <P2 as Query<T2>>::identity())
    }

    fn combine(lhs: &(T1, T2), rhs: &(T1, T2)) -> (T1, T2) {
        (
            <P1 as Query<T1>>::combine(&lhs.0, &rhs.0),
            <P2 as Query<T2>>::combine(&lhs.1, &rhs.1),
        )
    }
}

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
pub trait MonoidAction {
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

    /// Returns the identity mapping.
    fn identity() -> Self::Map;

    /// Combines the two maps and returns the result.
    ///
    /// If the operation is **not** commutative, be careful of the order of elements.
    fn combine(lhs: &Self::Map, rhs: &Self::Map) -> Self::Map;

    /// Applies the mapping on the element and returns the result.
    ///
    /// If size of each segment is required, add it to each element, **not** mapping.
    fn apply(mapping: &Self::Map, element: &Self::Set) -> Self::Set;
}
