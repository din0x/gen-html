//! Demarcating edits elements.
//!
//! These elements let you provide indications that specific parts of the text have been altered.
//!
//! [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element#demarcating_edits)

// TODO: Add `datetime` attribute.

use crate::{generate::create_element, NonInteractiveContent, PhrasingContent};

create_element! {
    "The `<ins>` HTML element represents a range of text that has been added to a document. You can use the [`<del>`](del) element to similarly represent a range of text that has been deleted from the document.",
    pub (Ins, ins): [],
    []
}

create_element! {
    "The `<del>` HTML element represents a range of text that has been deleted from a document. This can be used when rendering \"track changes\" or source code diff information, for example. The [`<ins>`](ins) element can be used for the opposite purpose: to indicate text that has been added to the document.",
    pub (Del, del): [],
    []
}
