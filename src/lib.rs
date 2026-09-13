//! Provides:
//! 1. basic `Future`s that serve as building blocks for users;
//! 2. combinators, that allow combining `Future`s;
//! 3. executor to await the top-level `Future`.

pub mod basic_futures;
pub mod combinators;
pub mod executor;
