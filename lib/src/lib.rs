//!
//!
//#![doc = include_str!("../README.md")]
//!

pub mod core;

#[cfg(feature = "text")]
pub mod text;

#[cfg(feature = "markdown")]
pub mod markdown;

#[cfg(feature = "html")]
pub mod html;

#[cfg(feature = "pdf")]
pub mod pdf;
