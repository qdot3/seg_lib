/// Defines query on segment tree variants.
pub trait Query<T> {
    fn identity() -> T;
    fn combine(lhs: &T, rhs: &T) -> T;
}

/// Defines update query on segment tree variants.
pub trait Update<T> {
    type Arg;

    fn identity() -> T;
    fn combine(previous: &T, new: &T) -> T;
    fn update(op: &T, arg: &Self::Arg) -> Self::Arg;
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
