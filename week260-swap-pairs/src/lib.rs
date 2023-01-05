//! Week 260
//!
//! "__Given a list, swap every two adjacent nodes.__ Something to think about: How would your code
//! change if this were a linked list, versus an array?"\
//!
//! Two solutions to this week's problem; one that only works on `slice`s and one that can consume any iterable.
//!
///
/// Hard Mode: `iters::SwapPairs`
///
/// - Works on anything that can be coverted into an `Iterator` with `IntoIterator`.
/// - No restrictions on `T`.
/// - Streamable (can work with non-terminating iterators).
///
pub mod iters;
/// Easy mode: `slice::SwapPairs`
///
/// - Swaps pairs in any `&mut [T]` in-place.
/// - No restrictions on `T` (doesn't need `Copy` or `Clone`).
/// - Simplest implementation, and no memory overhead!
/// - Only works on sliceable types.
///
pub mod slice;
