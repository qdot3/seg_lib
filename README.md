# seg_lib

[![crates.io](https://img.shields.io/crates/v/seg_lib.svg)](https://crates.io/crates/seg_lib)
[![docs.rs](https://docs.rs/seg_lib/badge.svg)](https://docs.rs/seg_lib)
[![verify](https://github.com/qdot3/seg_lib/workflows/verify/badge.svg)](https://github.com/qdot3/seg_lib/actions)

A collection of segment tree variants.

## Usage

Run this command in your project directory:

```text
cargo add seg_lib
```

## Guide

|                            | range query | range update | note                                |
| -------------------------- | ----------- | ------------ | ----------------------------------- |
| [`SegmentTree`]            | ✅           | ❌            |                                     |
| [`DynamicSegmentTree`]     | ✅           | ❌            | large array                         |
| [`DualSegmentTree`]        | ❌           | ✅            |                                     |
| [`LazySegmentTree`]        | ✅           | ✅            |                                     |
| [`DynamicLazySegmentTree`] | ✅           | ✅            | large array                         |
| [`AssignSegmentTree`]      | ✅           | ✅            | specialized for range assign update |

## License

Licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.
