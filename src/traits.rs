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
