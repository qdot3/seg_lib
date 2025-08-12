use crate::traits::Update;

#[derive(Debug)]
pub struct Assign;

impl<T> Update<Option<T>> for Assign
where
    T: Clone,
{
    type Set = T;

    fn identity() -> Option<T> {
        None
    }

    fn combine(_previous: &Option<T>, new: &Option<T>) -> Option<T> {
        new.clone()
    }

    fn update(op: &Option<T>, arg: &Self::Set) -> Self::Set {
        match op {
            Some(value) => value.clone(),
            None => arg.clone(),
        }
    }
}
