//! **arrayvec** provides the types [`ArrayVec`] and [`ArrayString`]: 
//! array-backed vector and string types, which store their contents inline.
//!
//! The arrayvec package has the following cargo features:
//!
//! - `std`
//!   - Optional, enabled by default
//!   - Use libstd; disable to use `no_std` instead.
//!
//! - `serde`
//!   - Optional
//!   - Enable serialization for ArrayVec and ArrayString using serde 1.x
//!
//! ## Example
//! ```rust
//! use arrayvec::{ArrayVec, CapacityError};
//! 
//! // Creates a new ArrayVec with a capacity of 3 and contents [1, 2, 3].
//! let mut stack = ArrayVec::from([1, 2, 3]);
//! assert_eq!(stack.pop(), Some(3));
//! stack.push(4);
//! 
//! // Now the stack is full:
//! assert!(stack.is_full());
//! assert_eq!(stack.try_push(5), Err(CapacityError::new(5)))
//! ```
//! 
//! ## Rust Version
//!
//! This version of arrayvec requires Rust 1.51 or later.
//!
#![doc(html_root_url="https://docs.rs/arrayvec/0.7/")]
#![cfg_attr(not(feature="std"), no_std)]

#[cfg(feature="serde")]
extern crate serde;

#[cfg(not(feature="std"))]
extern crate core as std;

pub(crate) type LenUint = u32;

macro_rules! assert_capacity_limit {
    ($cap:expr) => {
        if std::mem::size_of::<usize>() > std::mem::size_of::<LenUint>() {
            if $cap > LenUint::MAX as usize {
                panic!("ArrayVec: largest supported capacity is u32::MAX")
            }
        }
    }
}

macro_rules! assert_capacity_limit_const {
    ($cap:expr) => {
        if std::mem::size_of::<usize>() > std::mem::size_of::<LenUint>() {
            if $cap > LenUint::MAX as usize {
                [/*ArrayVec: largest supported capacity is u32::MAX*/][$cap]
            }
        }
    }
}

mod arrayvec_impl;
mod arrayvec;
mod array_string;
mod char;
mod errors;
mod utils;

pub use crate::array_string::ArrayString;
pub use crate::errors::CapacityError;

pub use crate::arrayvec::{ArrayVec, IntoIter, Drain};
