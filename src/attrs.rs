use std::{borrow::Cow, collections::BTreeMap, fmt};

pub struct Attrs {
    attrs: BTreeMap<Cow<'static, str>, Cow<'static, str>>,
}

impl Attrs {
    pub fn new() -> Self {
        Self {
            attrs: BTreeMap::new(),
        }
    }

    pub fn insert(
        &mut self,
        attr: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
    ) {
        self.attrs.insert(attr.into(), value.into());
    }
}

impl fmt::Display for Attrs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.attrs
            .iter()
            .map(|(k, v)| {
                if v.is_empty() {
                    write!(f, " {}", k,)
                } else {
                    let escaped = html_escape::encode_double_quoted_attribute(&v);

                    write!(f, " {}=\"{}\"", k, escaped)
                }
            })
            .collect()
    }
}
