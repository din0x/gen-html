#![feature(macro_metavar_expr, type_changing_struct_update)]
#![doc = include_str!("../README.md")]

pub mod content;
pub mod inline_text;
pub mod edits;

pub mod attribute;
mod attrs;
pub mod element;
mod marker;
pub mod root;

#[cfg(feature = "css-style")]
pub use css_style;

pub use marker::{Flow, Html, NonInteractive, Phrasing};
