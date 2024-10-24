//! The `rel` HTML attribute values.
//!  
//! # Example
//! ```
//! use gen_html::{inline_text::a, attribute::rel::*};
//!
//! let html = a("hello").rel(Me);
//! ```

use core::fmt;

use derive_more::derive::Display;

macro_rules! generate_rel {
    [$(($name:ident, $($into:ident),*)),*] => {
        $(
            pub struct $name;

            $(
                impl From<$name> for concat_idents!($into, List) {
                    fn from(_: $name) -> Self {
                        Self { data: vec![$into::$name] }
                    }
                }

                impl From<$name> for $into {
                    fn from(_: $name) -> Self {
                        Self::$name
                    }
                }
            )*
        )*
    };
}

generate_rel![
    (Alternate, LinkRel, AnchorRel),
    (Author, LinkRel, AnchorRel),
    (Bookmark, AnchorRel),
    (Canonical, LinkRel),
    (DnsPrefetch, LinkRel),
    (Expect, LinkRel),
    (External, AnchorRel, FormRel),
    (Help, LinkRel, AnchorRel, FormRel),
    (Icon, LinkRel),
    (License, LinkRel, AnchorRel, FormRel),
    (Manifest, LinkRel),
    (Me, LinkRel, AnchorRel),
    (ModulePreload, LinkRel),
    (Next, LinkRel, AnchorRel, FormRel),
    (NoFollow, AnchorRel, FormRel),
    (NoOpener, AnchorRel, FormRel),
    (NoReferrer, AnchorRel, FormRel),
    (Opener, AnchorRel, FormRel),
    (PingBack, LinkRel),
    (Preconnect, LinkRel),
    (Prefetch, LinkRel),
    (Preload, LinkRel),
    (Prerender, LinkRel),
    (Prev, LinkRel, AnchorRel, FormRel),
    (PrivacyPolicy, LinkRel, AnchorRel),
    (Search, LinkRel, AnchorRel, FormRel),
    (StyleSheet, LinkRel),
    (Tag, AnchorRel),
    (TermsOfService, LinkRel, AnchorRel)
];

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinkRelList {
    data: Vec<LinkRel>,
}

impl fmt::Display for LinkRelList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.iter().enumerate().try_for_each(|(i, e)| {
            if i != 0 {
                write!(f, ",")?;
            }

            write!(f, "{e}")
        })
    }
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LinkRel {
    #[display("alternate")]
    Alternate,
    #[display("author")]
    Author,
    #[display("canonical")]
    Canonical,
    #[display("dns-prefetch")]
    DnsPrefetch,
    #[display("expect")]
    Expect,
    #[display("help")]
    Help,
    #[display("icon")]
    Icon,
    #[display("license")]
    License,
    #[display("manifest")]
    Manifest,
    #[display("me")]
    Me,
    #[display("modulepreload")]
    ModulePreload,
    #[display("next")]
    Next,
    #[display("pingback")]
    PingBack,
    #[display("preconnect")]
    Preconnect,
    #[display("prefetch")]
    Prefetch,
    #[display("preload")]
    Preload,
    #[display("prerender")]
    Prerender,
    #[display("prev")]
    Prev,
    #[display("privacy-policy")]
    PrivacyPolicy,
    #[display("search")]
    Search,
    #[display("stylesheet")]
    StyleSheet,
    #[display("terms-of-service")]
    TermsOfService,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnchorRelList {
    data: Vec<AnchorRel>,
}

impl fmt::Display for AnchorRelList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.iter().enumerate().try_for_each(|(i, e)| {
            if i != 0 {
                write!(f, ",")?;
            }

            write!(f, "{e}")
        })
    }
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnchorRel {
    #[display("alternate")]
    Alternate,
    #[display("author")]
    Author,
    #[display("bookmark")]
    Bookmark,
    #[display("external")]
    External,
    #[display("help")]
    Help,
    #[display("license")]
    License,
    #[display("me")]
    Me,
    #[display("next")]
    Next,
    #[display("nofollow")]
    NoFollow,
    #[display("noopener")]
    NoOpener,
    #[display("noreferrer")]
    NoReferrer,
    #[display("opener")]
    Opener,
    #[display("prev")]
    Prev,
    #[display("privacy-policy")]
    PrivacyPolicy,
    #[display("search")]
    Search,
    #[display("tag")]
    Tag,
    #[display("terms-of-service")]
    TermsOfService,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FormRelList {
    data: Vec<FormRel>,
}

impl fmt::Display for FormRelList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.iter().enumerate().try_for_each(|(i, e)| {
            if i != 0 {
                write!(f, ",")?;
            }

            write!(f, "{e}")
        })
    }
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FormRel {
    #[display("external")]
    External,
    #[display("help")]
    Help,
    #[display("license")]
    License,
    #[display("next")]
    Next,
    #[display("nofollow")]
    NoFollow,
    #[display("noopener")]
    NoOpener,
    #[display("noreferrer")]
    NoReferrer,
    #[display("opener")]
    Opener,
    #[display("prev")]
    Prev,
    #[display("search")]
    Search,
}

macro_rules! impl_tuple_from {
    ($rel:ident, $inner:ident) => {
        impl From<()> for $rel {
            fn from(_: ()) -> Self {
                Self { data: Vec::new() }
            }
        }
    };
    ($rel:ident, $inner:ident, $( $t:ident ),*) => {
        impl<$($t: Into<$inner>),*> From<($($t,)*)> for $rel {
            fn from(value: ($($t,)*)) -> Self {
                let mut data = Vec::new();

                $(
                    {
                        let value = value.${index()}.into();

                        if !data.contains(&value) {
                            data.push(value);
                        }
                    }

                    ${ignore($t)}
                )*

                Self { data }
            }
        }
    };
}

macro_rules! impl_rel_from {
    ($rel:ident, $inner:ident) => {
        impl_tuple_from!($rel, $inner);
        impl_tuple_from!($rel, $inner, T0);
        impl_tuple_from!($rel, $inner, T0, T1);
        impl_tuple_from!($rel, $inner, T0, T1, T2);
        impl_tuple_from!($rel, $inner, T0, T1, T2, T3);
        impl_tuple_from!($rel, $inner, T0, T1, T2, T3, T4);
        impl_tuple_from!($rel, $inner, T0, T1, T2, T3, T4, T5);
        impl_tuple_from!($rel, $inner, T0, T1, T2, T3, T4, T5, T6);
        impl_tuple_from!($rel, $inner, T0, T1, T2, T3, T4, T5, T6, T7);
        impl_tuple_from!($rel, $inner, T0, T1, T2, T3, T4, T5, T6, T7, T8);
        impl_tuple_from!($rel, $inner, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
        impl_tuple_from!($rel, $inner, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
        impl_tuple_from!($rel, $inner, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
        impl_tuple_from!($rel, $inner, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
        impl_tuple_from!($rel, $inner, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
        impl_tuple_from!(
            $rel, $inner, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14
        );
        impl_tuple_from!(
            $rel, $inner, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15
        );
    };
}

impl_rel_from!(LinkRelList, LinkRel);
impl_rel_from!(AnchorRelList, AnchorRel);
impl_rel_from!(FormRelList, FormRel);

macro_rules! impl_attribute_for_rel {
    ($($rel:ident),*) => {
        $(
            impl $crate::attribute::Attribute for $rel {
                fn render_attr(&self, attr: &str, target: &mut dyn std::fmt::Write) -> std::fmt::Result {
                    if self.data.is_empty() {
                        return Ok(());
                    }

                    write!(target, " {attr}=\"{}\"", self)
                }
            }
        )*
    };
}

impl_attribute_for_rel!(LinkRelList, AnchorRelList, FormRelList);
