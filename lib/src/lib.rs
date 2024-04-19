//!
#![doc = include_str!("C:/Users/Lucky/RustroverProjects/shiva/README.md")]
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
