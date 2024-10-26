//! Text content elements.
//!
//! Use HTML text content elements to organize blocks or sections of content
//! placed between the opening `<body>` and closing `</body>` tags. Important
//! for [accessibility] and [SEO], these elements identify the purpose or structure
//! of that content.
//!
//! [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element#text_content)
//!
//! [accessibility]: https://developer.mozilla.org/en-US/docs/Glossary/Accessibility
//! [SEO]: https://developer.mozilla.org/en-US/docs/Glossary/SEO

use std::borrow::Cow;

use crate::{
    element::flow_elem, generate::create_element, FlowContent, NonInteractiveContent,
    PhrasingContent,
};

// TODO: Add `<dd>`, `<dl>`, `<dt>`.
// TODO: Add `<figure>, `<figcaption>`.
// TODO: Add `<hr>`.
// TODO: Add `<menu>`, `<ol>`, `<ul>`, `<li>`.

create_element!("# Example\n```\nuse gen_html::{Render, text_content::{blockquote, p}};\n\nlet html = blockquote(\n    p(\"Avian carriers can provide high delay, low throughput, and low altitude\")\n).cite(\"https://datatracker.ietf.org/doc/html/rfc1149\");\n```",
pub (BlockQuote, blockquote): [FlowContent], [
    {
        cite, cite,
        Option<Cow<'static, str>>,
        {
            /// Sets the `cite` attribute.
            pub fn cite(mut self, cite: impl Into<Cow<'static, str>>) -> Self {
                self.cite = Some(cite.into());
                self
            }
        }
    }
]);

impl<T: FlowContent> FlowContent for BlockQuote<T> {}
impl<T: FlowContent + NonInteractiveContent> NonInteractiveContent for BlockQuote<T> {}

flow_elem!(pub Div, div, FlowContent, text_content);
flow_elem!(pub P, p, PhrasingContent, text_content);
flow_elem!(pub Pre, pre, PhrasingContent, text_content);
