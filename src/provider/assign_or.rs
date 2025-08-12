use std::marker::PhantomData;

use crate::traits::Update;

#[derive(Debug)]
pub struct AssignOrProvider;

pub enum AssignOr<T, U, UpdateProvider>
where
    UpdateProvider: Update<U, Set = T>,
{
    Assign(T),
    Or(U, PhantomData<UpdateProvider>),
}

impl<T, U, UpdateProvider> Update<AssignOr<T, U, UpdateProvider>> for AssignOrProvider
where
    T: Clone,
    UpdateProvider: Update<U, Set = T>,
{
    type Set = T;

    fn identity() -> AssignOr<T, U, UpdateProvider> {
        AssignOr::Or(<UpdateProvider as Update<U>>::identity(), PhantomData)
    }

    fn combine(
        previous: &AssignOr<T, U, UpdateProvider>,
        new: &AssignOr<T, U, UpdateProvider>,
    ) -> AssignOr<T, U, UpdateProvider> {
        match (previous, new) {
            (_, AssignOr::Assign(new)) => AssignOr::Assign(new.clone()),
            (AssignOr::Assign(arg), AssignOr::Or(op, _)) => {
                AssignOr::Assign(<UpdateProvider as Update<U>>::update(op, arg))
            }
            (AssignOr::Or(previous, _), AssignOr::Or(new, _)) => AssignOr::Or(
                <UpdateProvider as Update<U>>::combine(previous, new),
                PhantomData,
            ),
        }
    }

    fn update(op: &AssignOr<T, U, UpdateProvider>, arg: &Self::Set) -> Self::Set {
        match op {
            AssignOr::Assign(new) => new.clone(),
            AssignOr::Or(op, _) => <UpdateProvider as Update<U>>::update(op, arg),
        }
    }
}
