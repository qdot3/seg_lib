use crate::{Monoid, ops::Assign};

pub enum AssignOr<M>
where
    M: Monoid,
    <M as Monoid>::Set: Clone,
{
    Assign(<Assign<<M as Monoid>::Set> as Monoid>::Set),
    Other(<M as Monoid>::Set),
}

impl<M> Monoid for AssignOr<M>
where
    M: Monoid,
    <M as Monoid>::Set: Clone,
{
    type Set = Self;

    const IS_COMMUTATIVE: bool = false;

    fn identity() -> Self::Set {
        Self::Assign(Assign::identity())
    }

    fn combine(lhs_or_prev: &Self::Set, rhs_or_new: &Self::Set) -> Self::Set {
        match (lhs_or_prev, rhs_or_new) {
            (AssignOr::Assign(lhs_or_prev), AssignOr::Assign(rhs_or_new)) => {
                Self::Assign(Assign::combine(lhs_or_prev, rhs_or_new))
            }
            (AssignOr::Assign(lhs_or_prev), AssignOr::Other(rhs_or_new)) => match lhs_or_prev {
                Some(lhs_or_prev) => {
                    Self::Assign(Some(<M as Monoid>::combine(lhs_or_prev, rhs_or_new)))
                }
                None => Self::Other(rhs_or_new.clone()),
            },
            (AssignOr::Other(lhs_or_prev), AssignOr::Assign(rhs_or_new)) => match rhs_or_new {
                Some(rhs_or_new) => Self::Assign(Some(rhs_or_new.clone())),
                None => Self::Other(lhs_or_prev.clone()),
            },
            (AssignOr::Other(lhs_or_prev), AssignOr::Other(rhs_or_new)) => {
                Self::Other(<M as Monoid>::combine(lhs_or_prev, rhs_or_new))
            }
        }
    }
}
