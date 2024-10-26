macro_rules! elem {
    ($doc:expr, $vis:vis $name:ident, $tag:ident, $content:tt, $($category:ty),*) => {
        $crate::generate::create_element!{
            $doc,
            $vis ($name, $tag): [$content],
            []
        }

        $(
            impl<T: $content> $category for $name<T> {}
        )*
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
