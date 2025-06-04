use proc_macro2::Span;
use quote::{ToTokens, quote_spanned};
use std::fmt;

pub struct Error {
    pub span: Span,
    pub kind: ErrorKind,
}

impl Error {
    pub fn new(span: Span, kind: ErrorKind) -> Self {
        Self { span, kind }
    }
}

impl ToTokens for Error {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let msg = self.kind.to_string();

        quote_spanned! {
            self.span => compile_error!(#msg);
        }
        .to_tokens(tokens);
    }
}

pub enum ErrorKind {
    InvalidTag(String),
    SelfClosing(String),
    NotSelfClosing(String),
    InvalidAttributeName(String),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTag(tag) => write!(f, "`{tag}` is not a valid HTML tag"),
            Self::SelfClosing(tag) => write!(f, "`{tag}` must be used like a self closing tag"),
            Self::NotSelfClosing(tag) => {
                write!(f, "`{tag}` cannot be used like a self closing tag")
            }
            Self::InvalidAttributeName(name) => write!(f, "`{name}` is not a valid attribute"),
        }
    }
}
