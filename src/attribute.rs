use core::fmt;
use std::{borrow::Cow, collections::HashMap};

use derive_more::derive::Display;

pub mod rel;

/// The `class` HTML attribute.
#[derive(Default)]
pub struct ClassList(Vec<Class>);

impl ClassList {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Attribute for ClassList {
    fn render_attr(&self, attr: &str, target: &mut dyn fmt::Write) -> fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        write!(target, " {attr}=\"{}\"", self)
    }
}

impl fmt::Display for ClassList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().try_for_each(|e| write!(f, "{e} "))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Class(Cow<'static, str>);

impl fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct InvalidClassName;

impl TryFrom<&'static str> for Class {
    type Error = InvalidClassName;

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        if value.chars().find(|ch| ch.is_whitespace()).is_some() {
            return Err(InvalidClassName);
        }

        Ok(Class(value.into()))
    }
}

impl From<&[Class]> for ClassList {
    fn from(value: &[Class]) -> Self {
        ClassList(value.to_vec())
    }
}

#[derive(Debug, Display, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[display("{_0}")]
pub struct Id(Cow<'static, str>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct InvalidId;

impl Id {
    /// Creates a new Id.
    ///
    /// # Panics
    /// If `id` contains whitespace.
    pub fn new(id: impl Into<Cow<'static, str>>) -> Self {
        Self::try_new(id).unwrap()
    }

    pub fn try_new(id: impl Into<Cow<'static, str>>) -> Result<Self, InvalidId> {
        let id = id.into();
        if id.contains(char::is_whitespace) {
            return Err(InvalidId);
        }

        Ok(Self(id))
    }
}

impl Attribute for Id {
    fn render_attr(&self, attr: &str, target: &mut dyn fmt::Write) -> fmt::Result {
        write!(target, " {attr}=\"{}\"", self.0)
    }
}

#[derive(Debug, Display, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[display("{lang}")]
pub struct Lang {
    lang: Cow<'static, str>,
}

impl Lang {
    pub fn new(lang: impl Into<Cow<'static, str>>) -> Self {
        Self { lang: lang.into() }
    }
}

impl Attribute for Lang {
    fn render_attr(&self, attr: &str, target: &mut dyn fmt::Write) -> fmt::Result {
        write!(
            target,
            " {attr}=\"{}\"",
            html_escape::encode_double_quoted_attribute(&self.lang)
        )
    }
}

/// The `target` HTML attribute.
///
/// The `target` attribute specifies where to open the linked document.
#[non_exhaustive]
#[derive(Debug, Display)]
pub enum Target {
    /// Opens the linked document in a new window or tab.
    #[display("_blank")]
    Blank,
    /// Opens the linked document in the same frame as it was clicked (default).
    #[display("self_")]
    Self_,
    /// Opens the linked document in the parent frame.
    #[display("_parent")]
    Parent,
    /// Opens the linked document in the full body of the window.
    #[display("_top")]
    Top,
    // TODO: Add FrameName
}

impl From<Target> for Cow<'static, str> {
    fn from(value: Target) -> Self {
        match value {
            Target::Blank => Cow::Borrowed("_blank"),
            Target::Self_ => Cow::Borrowed("_self"),
            Target::Parent => Cow::Borrowed("_parent"),
            Target::Top => Cow::Borrowed("_top"),
        }
    }
}

impl Attribute for Target {
    fn render_attr(&self, attr: &str, target: &mut dyn fmt::Write) -> fmt::Result {
        let s = match self {
            Target::Blank => "_blank",
            Target::Self_ => "_self",
            Target::Parent => "_parent",
            Target::Top => "_top",
        };

        write!(target, " {attr}=\"{s}\"")
    }
}

pub struct Download(Option<Cow<'static, str>>);

impl From<Option<Cow<'static, str>>> for Download {
    fn from(value: Option<Cow<'static, str>>) -> Self {
        Self(value)
    }
}

impl Attribute for Download {
    fn render_attr(&self, attr: &str, target: &mut dyn fmt::Write) -> fmt::Result {
        let Some(name) = self.0.as_ref() else {
            return write!(target, " {attr}");
        };

        write!(
            target,
            "{attr}=\"{}\"",
            html_escape::encode_double_quoted_attribute(name)
        )
    }
}

#[derive(Debug, Display, Clone, Copy)]
pub enum ARel {
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
    #[display("next")]
    Next,
    #[display("nofollow")]
    NoFollow,
    #[display("noopener")]
    NoOpener,
    #[display("noreferrer")]
    NoReferrer,
    #[display("prev")]
    Prev,
    #[display("search")]
    Search,
    #[display("tag")]
    Tag,
}

pub(crate) trait Attribute {
    fn render_attr(&self, attr: &str, target: &mut dyn fmt::Write) -> fmt::Result;
}

impl Attribute for Cow<'static, str> {
    fn render_attr(&self, attr: &str, target: &mut dyn fmt::Write) -> fmt::Result {
        write!(
            target,
            " {attr}=\"{}\"",
            html_escape::encode_double_quoted_attribute(self)
        )
    }
}

impl Attribute for u32 {
    fn render_attr(&self, attr: &str, target: &mut dyn fmt::Write) -> fmt::Result {
        write!(target, " {attr}=\"{}\"", self)
    }
}

impl Attribute for bool {
    fn render_attr(&self, attr: &str, target: &mut dyn fmt::Write) -> fmt::Result {
        if *self {
            write!(target, " {attr}")?;
        }

        Ok(())
    }
}

impl<T: Attribute> Attribute for Option<T> {
    fn render_attr(&self, attr: &str, target: &mut dyn fmt::Write) -> fmt::Result {
        let Some(inner) = self else { return Ok(()) };

        inner.render_attr(attr, target)
    }
}

#[derive(Default)]
pub(crate) struct DataMap {
    data: HashMap<Cow<'static, str>, Cow<'static, str>>,
}

impl DataMap {
    pub(crate) fn insert(&mut self, key: Cow<'static, str>, value: Cow<'static, str>) {
        if key
            .split('-')
            .map(|seg| seg.is_empty() || seg.contains(|ch: char| !ch.is_ascii_lowercase()))
            .filter(|invalid| *invalid)
            .count()
            > 0
        {
            panic!("`key` must be kebab-case")
        }

        self.data.insert(key, value);
    }
}

impl Attribute for DataMap {
    fn render_attr(&self, _: &str, target: &mut dyn fmt::Write) -> fmt::Result {
        self.data.iter().try_for_each(|(k, v)| {
            write!(
                target,
                " data-{k}=\"{}\"",
                html_escape::encode_double_quoted_attribute(v)
            )
        })
    }
}
