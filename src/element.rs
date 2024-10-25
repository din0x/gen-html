//! HTML5 element builders.

use std::{borrow::Cow, fmt};

use crate::{
    attribute::Target,
    attrs::Attrs,
    marker::{FlowContent, NonInteractiveContent, PhrasingContent, Render},
};

macro_rules! elem {
    ($doc:expr, $vis:vis $name:ident, $tag:ident, $content:tt, $($category:ty),*) => {
        #[doc = concat!("Builder for the `<", stringify!($tag), ">` HTML element.\n",)]
        #[doc = $doc]
        #[derive(Default)]
        $vis struct $name<T: $content> {
            content: T,
            class: $crate::attribute::ClassList,
            id: Option<$crate::attribute::Id>,
            lang: Option<$crate::attribute::Lang>,
        }

        impl<T: $content> $name<T> {
            /// Sets the HTML `class` attribute.
            pub fn class(mut self, class: $crate::attribute::ClassList) -> Self {
                self.class = class;
                self
            }

            /// Sets the HTML `id` attribute.
            pub fn id(mut self, id: $crate::attribute::Id) -> Self {
                self.id = Some(id);
                self
            }

            /// Sets the HTML `lang` attribute.
            pub fn lang(mut self, lang: $crate::attribute::Lang) -> Self {
                self.lang = Some(lang);
                self
            }
        }

        impl<T: $content> $crate::Render for $name<T> {
            fn render(self, target: &mut dyn std::fmt::Write) -> std::fmt::Result {
                write!(target, concat!("<", stringify!($tag)))?;

                if !self.class.is_empty() {
                    write!(target, " class=\"{}\"", self.class)?;
                }

                self.id.as_ref().map(|id| write!(target, " id=\"{id}\""));
                self.lang.as_ref().map(|lang| write!(target, " lang=\"{lang}\""));

                write!(target, ">")?;
                self.content.render(target)?;
                write!(target, concat!("</", stringify!($tag), ">"))
            }
        }

        $(
            impl<T: $content> $category for $name<T> {}
        )*

        #[doc = concat!("The `<", stringify!($tag), ">` HTML element.\n",)]
        #[doc = $doc]
        $vis fn $tag<T: $content>(content: T) -> $name<T> {
            $name { content, ..$name::<()>::default() }
        }
    };
    ($vis:vis $name:ident, $tag:ident, $content:tt, $group:ident, $($category:ty),*) => {
        $crate::element::elem!(concat!(
            "# Examples\n",
            "```\nuse gen_html::{Render, ", stringify!($group), "::", stringify!($tag), "};\n\n",
            "let html = ", stringify!($tag), "(\"Some text\");\n",
            "assert_eq!(html.render_to_string(), \"<", stringify!($tag), ">Some text</", stringify!($tag), ">\");"), $vis $name, $tag, $content, $($category),*);
    };
}

pub(crate) use elem;

macro_rules! non_interactive {
    ($name:ident, $content:tt) => {
        impl<T: $content + $crate::NonInteractiveContent> $crate::NonInteractiveContent
            for $name<T>
        {
        }
    };
}

pub(crate) use non_interactive;

macro_rules! flow_elem {
    ($vis:vis $name:ident, $tag:ident, $content:tt, $group:ident) => {
        $crate::element::elem!($vis $name, $tag, $content, $group, $crate::FlowContent);
        $crate::element::non_interactive!($name, $content);
    };
}

pub(crate) use flow_elem;

flow_elem!(pub Div, div, FlowContent, element);

flow_elem!(pub P, p, PhrasingContent, element);

macro_rules! phrasing_elem {
    ($vis:vis $name:ident, $tag:ident, $content:tt, $group:ident) => {
        $crate::element::elem!($vis $name, $tag, $content, $group, $crate::FlowContent, $crate::PhrasingContent);
        $crate::element::non_interactive!($name, $content);
    };
}

pub(crate) use phrasing_elem;

/// Builder for the `<a>` HTML element.
///
/// # Examples
/// ```
/// use gen_html::{Render, element::a};
///
/// let html = a("Click me", "https://some-url.com");
/// assert_eq!(html.render_to_string(), "<a href=\"https://some-url.com\">Click me</a>");
/// ```
pub struct A<T: PhrasingContent + NonInteractiveContent> {
    content: T,
    attrs: Attrs,
}

impl<T: PhrasingContent + NonInteractiveContent> A<T> {
    pub fn target(mut self, target: Target) -> Self {
        self.attrs.insert("target", target);
        self
    }
}

impl<T: PhrasingContent + NonInteractiveContent> Render for A<T> {
    fn render(self, target: &mut dyn std::fmt::Write) -> std::fmt::Result {
        write!(target, "<a{}>", self.attrs)?;
        self.content.render(target)?;
        write!(target, "</a>")
    }
}

impl<T: PhrasingContent + NonInteractiveContent> FlowContent for A<T> {}

impl<T: PhrasingContent + NonInteractiveContent> PhrasingContent for A<T> {}

/// The `<a>` HTML element.
///
/// # Examples
/// ```
/// use gen_html::{Render, element::a};
///
/// let html = a("Click me", "https://some-url.com");
/// assert_eq!(html.render_to_string(), "<a href=\"https://some-url.com\">Click me</a>");
/// ```
pub fn a<T: PhrasingContent + NonInteractiveContent>(
    content: T,
    href: impl Into<Cow<'static, str>>,
) -> A<T> {
    let mut attrs = Attrs::new();
    attrs.insert("href", href);

    A { content, attrs }
}

/// Construct HTML from an iterator.
///
/// Use [`iter`] and [`Iter`] to construct HTML element from an iterator.
///
/// # Examples
/// ```
/// use gen_html::{Render, element::{iter, p}};
///
/// let html = iter((0..3).map(|i| p(format!("Paragraph {i}"))));
/// assert_eq!(html.render_to_string(), "<p>Paragraph 0</p><p>Paragraph 1</p><p>Paragraph 2</p>");
/// ```
pub struct Iter<I> {
    iter: I,
}

/// Construct HTML from an iterator.
///
/// Use [`iter`] and [`Iter`] to construct HTML element from an iterator.
///
/// # Examples
/// ```
/// use gen_html::{Render, element::{iter, p}};
///
/// let html = iter((0..3).map(|i| p(format!("Paragraph {i}"))));
/// assert_eq!(html.render_to_string(), "<p>Paragraph 0</p><p>Paragraph 1</p><p>Paragraph 2</p>");
/// ```
pub fn iter<I: IntoIterator>(iter: I) -> Iter<I> {
    Iter { iter }
}

impl<I> Render for Iter<I>
where
    I: IntoIterator,
    I::Item: Render,
{
    fn render(self, target: &mut dyn fmt::Write) -> fmt::Result {
        for ele in self.iter {
            ele.render(target)?;
        }

        Ok(())
    }
}

impl<I> FlowContent for Iter<I>
where
    I: IntoIterator,
    I::Item: FlowContent,
{
}

impl<I> PhrasingContent for Iter<I>
where
    I: IntoIterator,
    I::Item: PhrasingContent,
{
}

impl<I> NonInteractiveContent for Iter<I>
where
    I: IntoIterator,
    I::Item: NonInteractiveContent,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FlowContent;

    #[test]
    fn p_is_flow() {
        require_flow(p(""));
    }

    fn require_flow(_: impl FlowContent) {}
}
