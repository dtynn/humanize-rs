//! This module provides some numeric traits used in parsing.

pub use num_traits::{cast, checked_pow, NumCast, NumOps, PrimInt};
use std::str::FromStr;

/// Represents the integer trait
pub trait Int: PrimInt + NumOps + FromStr {}

impl Int for i8 {}
impl Int for u8 {}
impl Int for i16 {}
impl Int for u16 {}
impl Int for i32 {}
impl Int for u32 {}
impl Int for i64 {}
impl Int for u64 {}
impl Int for isize {}
impl Int for usize {}

#[cfg(has_i128)]
impl Int for i128 {}
#[cfg(has_i128)]
impl Int for u128 {}
