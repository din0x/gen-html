use crate::Render;
use std::fmt::{self, Write};

/// Types that can be used as attributes in the [`html!`] macro.
///
/// Direct usage of this trait is discouraged. It's only supposed to be used by [`html!`] macro
/// internally.
///
/// [`html!`]: crate::html
pub trait Value {
    fn render_value_to(&self, name: &str, f: &mut fmt::Formatter) -> fmt::Result;
}

impl Value for bool {
    fn render_value_to(&self, name: &str, f: &mut fmt::Formatter) -> fmt::Result {
        if *self { write!(f, " {name}") } else { Ok(()) }
    }
}

impl<R: Render> Value for Option<R> {
    fn render_value_to(&self, name: &str, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(r) = self {
            write!(f, " {name}=\"")?;
            r.render_to(f)?;
            f.write_char('"')?;
        }

        Ok(())
    }
}

impl<R: Render> Value for R {
    fn render_value_to(&self, name: &str, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {name}=\"")?;
        self.render_to(f)?;
        f.write_char('"')?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Value;
    use std::fmt;

    #[test]
    fn bool_attributes() {
        let display = fmt::from_fn(|f| {
            false.render_value_to("checked", f).unwrap();
            true.render_value_to("some-attribute", f)
        });

        assert_eq!(display.to_string(), " some-attribute");
    }

    #[test]
    fn optional_attributes() {
        let display = fmt::from_fn(|f| {
            None::<i32>.render_value_to("hello-world", f).unwrap();
            Some("escape this\"<").render_value_to("attr-123", f)
        });

        assert_eq!(display.to_string(), r#" attr-123="escape this&quot;&lt;""#);
    }

    #[test]
    fn normal_attributes() {
        let display = fmt::from_fn(|f| {
            "&".render_value_to("hello-world", f).unwrap();
            "escape this\"<".render_value_to("attr-123", f)
        });

        assert_eq!(
            display.to_string(),
            r#" hello-world="&amp;" attr-123="escape this&quot;&lt;""#
        );
    }
}
