//! This library provides [`sk_error::Error`][SkError].
//! 
#![cfg_attr(not(test), no_std)]

mod error;
mod node;

pub type Result<T, E = ErrorRoot> = core::result::Result<T, E>;