//! The utilities of UNM that enable me to write UNM faster.
//!
//! It provides something like:
//!   - `iter`: The utilities for iterators. For example: [`iter::Slice::slice`]
//!   - `option`: The utilities for Option<T>. For example: [`option::UnwrapOrWithLog::unwrap_or_log`]
//!   - `val_inside`: A macro that check if a value is inside the values set.

#![warn(missing_docs)]

pub mod iter;
pub mod option;
mod val_inside;
