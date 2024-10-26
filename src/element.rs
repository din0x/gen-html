//! HTML5 element builders.

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

macro_rules! phrasing_elem {
    ($vis:vis $name:ident, $tag:ident, $content:tt, $group:ident) => {
        $crate::element::elem!($vis $name, $tag, $content, $group, $crate::FlowContent, $crate::PhrasingContent);
        $crate::element::non_interactive!($name, $content);
    };
}

pub(crate) use phrasing_elem;
