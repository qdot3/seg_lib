Creates a new instance over the given `range`,
initialized with [identity elements](crate::traits::Monoid::identity()).

Returns [`None`] if the range is empty.

If you know the total number of queries in advance, use [`with_capacity`](Self::with_capacity)
instead to avoid reallocations.
