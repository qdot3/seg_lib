use crate::traits::Update;

#[derive(Debug)]
pub struct AssignProvider;

impl<T> Update<Option<T>> for AssignProvider
where
    T: Clone,
{
    type Arg = T;

    fn identity() -> Option<T> {
        None
    }

    fn combine(_previous: &Option<T>, new: &Option<T>) -> Option<T> {
        new.clone()
    }

    fn update(op: &Option<T>, arg: &Self::Arg) -> Self::Arg {
        match op {
            Some(value) => value.clone(),
            None => arg.clone(),
        }
    }
}
