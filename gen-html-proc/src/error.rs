use proc_macro2::Span;
use quote::{ToTokens, quote_spanned};
use syn::Ident;

#[derive(Debug, Clone)]
pub enum Error {
    InvalidTag(Ident),
    SelfClosing(Ident),
    NotSelfClosing(Ident),
    AttributeSpecifiedMoreThenOnce { spans: Vec<Span>, key: String },
    Multi(Vec<Error>),
}

impl Error {
    pub fn empty() -> Self {
        Self::Multi(Vec::new())
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Multi(vec) if vec.is_empty())
    }

    pub fn push(&mut self, other: Self) {
        if let Self::Multi(v) = self {
            v.push(other);
        } else {
            *self = Self::Multi(vec![self.clone(), other])
        }
    }
}

impl ToTokens for Error {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::InvalidTag(tag) => {
                let msg = format!("`{tag}` is not a valid HTML tag");
                quote_spanned!(tag.span() => compile_error!(#msg);).to_tokens(tokens);
            }
            Self::SelfClosing(tag) => {
                let msg = format!("`{tag}` must be used like a self closing tag");
                quote_spanned!(tag.span() => compile_error!(#msg);).to_tokens(tokens);
            }
            Self::NotSelfClosing(tag) => {
                let msg = format!("`{tag}` cannot be used like a self closing tag");
                quote_spanned!(tag.span() => compile_error!(#msg);).to_tokens(tokens);
            }
            Self::AttributeSpecifiedMoreThenOnce { spans, key: name } => {
                let msg = format!("attribute `{name}` specified more then once");
                for span in spans {
                    quote_spanned!(*span => compile_error!(#msg);).to_tokens(tokens);
                }
            }
            Self::Multi(errors) => errors.iter().for_each(|e| e.to_tokens(tokens)),
        }
    }
}
