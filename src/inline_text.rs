//! Inline text elements.
//!
//! Use the HTML inline text semantic to define the meaning, structure, or style of a word, line,
//! or any arbitrary piece of text.
//!
//! [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element#inline_text_semantics)

use crate::{element::phrasing_elem, Flow, Phrasing};

// TODO: Implement `<abbr>`, `<a>`, <ruby>, `<data>`, <dfn>.
// TODO: Decide if we should implement `<b>`, `<i>`, `<s>`, <small>, <u> elements as they don't
// have semantic meaning.
// TODO: Decide if we should implement `<br>`.
// TODO: Add `<wbr>` tag.

phrasing_elem!(pub Span, span, Phrasing);

phrasing_elem!(pub Bdi, bdi, Phrasing);
phrasing_elem!(pub Bdo, bdo, Phrasing);

phrasing_elem!(pub Cite, cite, Phrasing);

phrasing_elem!(pub Code, code, Phrasing);
phrasing_elem!(pub Kbd, kbd, Phrasing);
phrasing_elem!(pub Samp, samp, Phrasing);
phrasing_elem!(pub Var, var, Phrasing);

phrasing_elem!(pub Em, em, Phrasing);
phrasing_elem!(pub Strong, strong, Phrasing);

phrasing_elem!(pub Mark, mark, Phrasing);
phrasing_elem!(pub Q, q, Phrasing);

phrasing_elem!(pub Sup, sup, Phrasing);
phrasing_elem!(pub Sub, sub, Phrasing);

// TODO: Add `datetime` attribute.
phrasing_elem!(pub Time, time, Phrasing);
