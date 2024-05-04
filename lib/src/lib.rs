//!
#![doc = include_str!("../README.md")]
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

#[cfg(feature = "json")]
pub mod json;

#[cfg(feature = "xml")]
pub mod xml;

#[cfg(feature = "csv")]
pub mod csv;

#[cfg(feature = "docx")]
pub mod docx;
