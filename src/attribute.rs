use core::fmt;
use std::borrow::Cow;

use derive_more::derive::Display;

#[derive(Default)]
pub struct ClassList(Vec<Class>);

impl ClassList {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
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
