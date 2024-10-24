//! Inline text elements.
//!
//! Use the HTML inline text semantic to define the meaning, structure, or style of a word, line,
//! or any arbitrary piece of text.
//!
//! [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element#inline_text_semantics)

use std::borrow::Cow;

use crate::{
    attribute::{rel::AnchorRelList, Download, Target},
    element::phrasing_elem,
    generate::create_element,
    FlowContent, NonInteractiveContent, PhrasingContent,
};

// TODO: Implement `<abbr>`, <ruby>, `<data>`, <dfn>.
// TODO: Decide if we should implement `<b>`, `<i>`, `<s>`, <small>, <u> elements as they don't
// have semantic meaning.
// TODO: Decide if we should implement `<br>`.
// TODO: Add `<wbr>` tag.

create_element! {
    "The `<a>` HTML element (or anchor element), with its href attribute, creates a hyperlink to web pages, files, email addresses, locations in the same page, or anything else a URL can address.\n\nContent within each `<a>` should indicate the link's destination. If the href attribute is present, pressing the enter key while focused on the `<a>` element will activate it.\n# Example\n```\nuse gen_html::{Render, inline_text::a};\n\nlet anchor = a(\"home\").href(\"/\");\nassert_eq!(anchor.render_to_string(), r#\"<a href=\"/\">home</a>\"#);\n```",
    pub (A, a): [PhrasingContent, NonInteractiveContent],
    [
        {
            href, href,
            Option<Cow<'static, str>>,
            {
                /// Sets the `href` attribute.
                ///
                /// # Example
                /// ```
                /// use gen_html::{Render, inline_text::a};
                ///
                /// let anchor = a("home").href("/");
                /// assert_eq!(anchor.render_to_string(), r#"<a href="/">home</a>"#);
                /// ```
                pub fn href(mut self, href: impl Into<Cow<'static, str>>) -> Self {
                    self.href = Some(href.into());
                    self
                }
            }
        },
        {
            download, download,
            Option<Download>,
            {
                /// Sets the `download` attribute.
                ///
                /// # Example
                /// ```
                /// use gen_html::{Render, inline_text::a};
                ///
                /// let anchor = a("hello.png").href("/assets/image_hello.png").download();
                /// ```
                pub fn download(mut self) -> Self {
                    self.download = Some(Download::from(None));
                    self
                }
            }
        },
        {
            target, target,
            Option<Target>,
            {
                /// Sets the `target` attribute.
                ///
                /// # Example
                /// ```
                /// use gen_html::{Render, inline_text::a, attribute::Target};
                ///
                /// let external_link = a("click me").href("/surprise.html").target(Target::Blank);
                /// let expected = r#"<a href="/surprise.html" target="_blank">click me</a>"#;
                ///
                /// assert_eq!(external_link.render_to_string(), expected);
                /// ```
                pub fn target(mut self, target: impl Into<Target>) -> Self {
                    self.target = Some(target.into());
                    self
                }
            }
        },
        {
            rel, rel,
            AnchorRelList,
            {
                /// Sets the `rel` attribute.
                pub fn rel(mut self, rel: impl Into<AnchorRelList>) -> Self {
                    self.rel = rel.into();
                    self
                }
            }
        }
]}

impl<T: PhrasingContent + NonInteractiveContent> PhrasingContent for A<T> {}
impl<T: PhrasingContent + NonInteractiveContent> FlowContent for A<T> {}

phrasing_elem!(pub Span, span, PhrasingContent, inline_text);

phrasing_elem!(pub Bdi, bdi, PhrasingContent, inline_text);
phrasing_elem!(pub Bdo, bdo, PhrasingContent, inline_text);

phrasing_elem!(pub Cite, cite, PhrasingContent, inline_text);

phrasing_elem!(pub Code, code, PhrasingContent, inline_text);
phrasing_elem!(pub Kbd, kbd, PhrasingContent, inline_text);
phrasing_elem!(pub Samp, samp, PhrasingContent, inline_text);
phrasing_elem!(pub Var, var, PhrasingContent, inline_text);

phrasing_elem!(pub Em, em, PhrasingContent, inline_text);
phrasing_elem!(pub Strong, strong, PhrasingContent, inline_text);

phrasing_elem!(pub Mark, mark, PhrasingContent, inline_text);
phrasing_elem!(pub Q, q, PhrasingContent, inline_text);

phrasing_elem!(pub Sup, sup, PhrasingContent, inline_text);
phrasing_elem!(pub Sub, sub, PhrasingContent, inline_text);

// TODO: Add `datetime` attribute.
phrasing_elem!(pub Time, time, PhrasingContent, inline_text);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Render;

    #[test]
    fn test_a_element() {
        let a = a("<a> <-- this is escaped")
            .href("<>\"")
            .download()
            .target(Target::Blank);

        let expected = r#"<a href="&lt;&gt;&quot;" download target="_blank">&lt;a&gt; &lt;-- this is escaped</a>"#;

        assert_eq!(a.render_to_string(), expected)
    }
}
