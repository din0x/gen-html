use crate::Render;
use std::fmt;

pub(crate) fn escape(s: &str, f: &mut fmt::Formatter) -> fmt::Result {
    use fmt::Write;

    for ch in s.chars() {
        match ch {
            '&' => write!(f, "&amp;")?,
            '<' => write!(f, "&lt;")?,
            '>' => write!(f, "&gt;")?,
            '"' => write!(f, "&quot;")?,
            ch => f.write_char(ch)?,
        }
    }

    Ok(())
}

/// Wrapper that escapes HTML special characters.
///
/// The following characters are escaped:
/// - `&` ==> `&amp;`
/// - `<` ==> `&lt;`
/// - `>` ==> `&gt;`
/// - `"` ==> `&quot;`
///
/// # Example
///
/// ```
/// use gen_html::Escaped;
/// assert_eq!(Escaped("&<>\"").to_string(), "&amp;&lt;&gt;&quot;")
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Escaped<T>(pub T);

impl<T: fmt::Display> fmt::Display for Escaped<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        escape(&self.0.to_string(), f)
    }
}

impl<T: fmt::Display> Render for Escaped<T> {
    fn render_to(&self, f: &mut fmt::Formatter) -> fmt::Result {
        escape(&self.0.to_string(), f)
    }
}
