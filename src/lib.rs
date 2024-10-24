#![feature(macro_metavar_expr, type_changing_struct_update, concat_idents)]
#![doc = include_str!("../README.md")]

pub mod content;
pub mod edits;
pub mod inline_text;

pub mod attribute;
mod attrs;
pub mod element;
mod marker;
pub mod root;

mod generate;

mod private {
    pub struct _Private;
}

#[cfg(feature = "css-style")]
pub use css_style;

pub use marker::{FlowContent, NonInteractiveContent, PhrasingContent, Render};
