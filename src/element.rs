//! HTML5 element builders.

use std::{borrow::Cow, fmt};

use crate::{
    attrs::Attrs,
    marker::{Flow, Html, NonInteractive, Phrasing},
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
            #[cfg(feature = "css-style")]
            style: $crate::css_style::Style,
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

            #[cfg(feature = "css-style")]
            pub fn style(mut self, style: $crate::css_style::Style) -> Self {
                self.style = style;
                self
            }
        }

        impl<T: $content> $crate::Html for $name<T> {
            fn fmt_html(self, target: &mut dyn std::fmt::Write) -> std::fmt::Result {
                write!(target, concat!("<", stringify!($tag)))?;

                if !self.class.is_empty() {
                    write!(target, " class=\"{}\"", self.class)?;
                }

                #[cfg(feature = "css-style")]
                {
                    let s = self.style.to_string();
                    if !s.is_empty() {
                        write!(target, " style=\"{}\"", s)?;
                    }
                }

                self.id.as_ref().map(|id| write!(target, " id=\"{id}\""));
                self.lang.as_ref().map(|lang| write!(target, " lang=\"{lang}\""));

                write!(target, ">")?;
                self.content.fmt_html(target)?;
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
    ($vis:vis $name:ident, $tag:ident, $content:tt, $($category:ty),*) => {
        $crate::element::elem!(concat!(
            "# Examples\n",
            "```\nuse gen_html::{Html, element::", stringify!($tag), "};\n\n",
            "let html = ", stringify!($tag), "(\"Some text\");\n",
            "assert_eq!(html.to_html_string(), \"<", stringify!($tag), ">Some text</", stringify!($tag), ">\");"), $vis $name, $tag, $content, $($category),*);
    };
}

pub(crate) use elem;

macro_rules! non_interactive {
    ($name:ident, $content:tt) => {
        impl<T: $content + $crate::NonInteractive> $crate::NonInteractive for $name<T> {}
    };
}

pub(crate) use non_interactive;

macro_rules! flow_elem {
    ($vis:vis $name:ident, $tag:ident, $content:tt) => {
        $crate::element::elem!($vis $name, $tag, $content, $crate::Flow);
        $crate::element::non_interactive!($name, $content);
    };
}

pub(crate) use flow_elem;

flow_elem!(pub Div, div, Flow);

flow_elem!(pub P, p, Phrasing);

macro_rules! phrasing_elem {
    ($vis:vis $name:ident, $tag:ident, $content:tt) => {
        $crate::element::elem!($vis $name, $tag, $content, Flow, Phrasing);
        $crate::element::non_interactive!($name, $content);
    };
}

pub(crate) use phrasing_elem;

/// Builder for the `<a>` HTML element.
///
/// # Examples
/// ```
/// use gen_html::{Html, element::a};
///
/// let html = a("Click me", "https://some-url.com");
/// assert_eq!(html.to_html_string(), "<a href=\"https://some-url.com\">Click me</a>");
/// ```
pub struct A<T: Phrasing + NonInteractive> {
    content: T,
    attrs: Attrs,
}

impl<T: Phrasing + NonInteractive> A<T> {
    pub fn target(mut self, target: Target) -> Self {
        self.attrs.insert("target", target);
        self
    }
}

#[non_exhaustive]
pub enum Target {
    Blank,
    Self_,
    Parent,
    Top,
    // TODO: Add FrameName
}

impl From<Target> for Cow<'static, str> {
    fn from(value: Target) -> Self {
        match value {
            Target::Blank => Cow::Borrowed("_blank"),
            Target::Self_ => Cow::Borrowed("_self"),
            Target::Parent => Cow::Borrowed("_parent"),
            Target::Top => Cow::Borrowed("_top"),
        }
    }
}

impl<T: Phrasing + NonInteractive> Html for A<T> {
    fn fmt_html(self, target: &mut dyn std::fmt::Write) -> std::fmt::Result {
        write!(target, "<a{}>", self.attrs)?;
        self.content.fmt_html(target)?;
        write!(target, "</a>")
    }
}

impl<T: Phrasing + NonInteractive> Flow for A<T> {}

impl<T: Phrasing + NonInteractive> Phrasing for A<T> {}

/// The `<a>` HTML element.
///
/// # Examples
/// ```
/// use gen_html::{Html, element::a};
///
/// let html = a("Click me", "https://some-url.com");
/// assert_eq!(html.to_html_string(), "<a href=\"https://some-url.com\">Click me</a>");
/// ```
pub fn a<T: Phrasing + NonInteractive>(content: T, href: impl Into<Cow<'static, str>>) -> A<T> {
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
/// use gen_html::{Html, element::{iter, p}};
///
/// let html = iter((0..3).map(|i| p(format!("Paragraph {i}"))));
/// assert_eq!(html.to_html_string(), "<p>Paragraph 0</p><p>Paragraph 1</p><p>Paragraph 2</p>");
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
/// use gen_html::{Html, element::{iter, p}};
///
/// let html = iter((0..3).map(|i| p(format!("Paragraph {i}"))));
/// assert_eq!(html.to_html_string(), "<p>Paragraph 0</p><p>Paragraph 1</p><p>Paragraph 2</p>");
/// ```
pub fn iter<I: IntoIterator>(iter: I) -> Iter<I> {
    Iter { iter }
}

impl<I> Html for Iter<I>
where
    I: IntoIterator,
    I::Item: Html,
{
    fn fmt_html(self, target: &mut dyn fmt::Write) -> fmt::Result {
        for ele in self.iter {
            ele.fmt_html(target)?;
        }

        Ok(())
    }
}

impl<I> Flow for Iter<I>
where
    I: IntoIterator,
    I::Item: Flow,
{
}

impl<I> Phrasing for Iter<I>
where
    I: IntoIterator,
    I::Item: Phrasing,
{
}

impl<I> NonInteractive for Iter<I>
where
    I: IntoIterator,
    I::Item: NonInteractive,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Flow;

    #[test]
    fn p_is_flow() {
        require_flow(p(""));
    }

    fn require_flow(_: impl Flow) {}
}
