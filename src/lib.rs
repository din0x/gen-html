#![feature(macro_metavar_expr, type_changing_struct_update, concat_idents)]
#![doc = include_str!("../README.md")]

pub mod content;
pub mod edits;
pub mod inline_text;
pub mod root;
pub mod text_content;

pub mod attribute;
mod attrs;
mod element;
mod marker;

mod generate;

mod private {
    pub struct _Private;
}

pub use marker::{FlowContent, NonInteractiveContent, PhrasingContent, Render};
