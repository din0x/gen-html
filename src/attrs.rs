use std::{borrow::Cow, collections::BTreeMap, fmt};

/// Valid HTML attribute name
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct AttrName(Cow<'static, str>);

impl AttrName {
    /// Creates valid attribute name
    ///
    /// # Panics
    /// If `name` is not a valid attribute
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        let name: Cow<_> = name.into();

        if name.contains(|ch| !char::is_ascii_lowercase(&ch) && ch != '-') {
            panic!("invalid attribute")
        }

        Self(name)
    }
}

#[derive(Debug, Clone)]
pub struct AttrValue(Cow<'static, str>);

pub struct Attrs {
    attrs: BTreeMap<AttrName, AttrValue>,
}

impl Attrs {
    pub fn new() -> Self {
        Self {
            attrs: BTreeMap::new(),
        }
    }

    pub fn set_lang(&mut self, lang: Cow<'static, str>) {
        self.attrs.insert(AttrName::new("lang"), AttrValue(lang));
    }

    pub fn set_class(&mut self, class: Class) {
        self.attrs
            .insert(AttrName::new("class"), AttrValue(class.0));
    }

    pub(crate) fn set_charset(&mut self, charset: Cow<'static, str>) {
        self.attrs
            .insert(AttrName::new("charset"), AttrValue(charset));
    }

    pub(crate) fn set_name(&mut self, name: Cow<'static, str>) {
        self.attrs.insert(AttrName::new("name"), AttrValue(name));
    }

    pub(crate) fn set_content(&mut self, content: Cow<'static, str>) {
        self.attrs
            .insert(AttrName::new("content"), AttrValue(content));
    }
}

impl fmt::Display for Attrs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.attrs
            .iter()
            .map(|(k, v)| {
                if v.0.is_empty() {
                    write!(f, " {}", k.0,)
                } else {
                    let escaped = html_escape::encode_double_quoted_attribute(&v.0);

                    write!(f, " {}=\"{}\"", k.0, escaped)
                }
            })
            .collect()
    }
}

pub struct Class(Cow<'static, str>);

pub trait IntoClass {
    fn into_class(self) -> Class;
}

impl<T: Into<Cow<'static, str>>> IntoClass for T {
    fn into_class(self) -> Class {
        Class(self.into())
    }
}
