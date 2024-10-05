macro_rules! gen_element {
    ($name:ident, $tag:ident, $content:ty, $example:expr, [$($types:ty),*]) => {
        #[doc = concat!("The `<", stringify!($tag), ">` HTML element.")]
        #[doc = "# Examples"]
        #[doc = $example]
        pub struct $name {
            attrs: $crate::attrs::Attrs,
            content: $crate::marker::Valid<$content>,
        }

        #[doc = concat!("Utility for creating `<", stringify!($tag), ">` element.\n\n")]
        #[doc = concat!("Use this function instead of [`", stringify!($name), "::new`]")]
        #[doc = "# Examples"]
        #[doc = $example]
        pub fn $tag(content: impl $crate::Element<$content>) -> $name {
            $name::new(content)
        }

        impl $name {
            #[doc = concat!("Creates a new `<", stringify!($tag), ">` element.")]
            pub fn new(content: impl $crate::marker::Element<$content>) -> Self {
                $name {
                    attrs: $crate::attrs::Attrs::new(),
                    content: content.into_html(),
                }
            }
        }

        $(
            impl $crate::Element<$types> for $name {
                fn into_html(self) -> $crate::Valid<$types> {
                    let s = format!(
                        "<{}{}>{}</{}>",
                        stringify!($tag),
                        self.attrs,
                        self.content.as_ref(),
                        stringify!($tag)
                    );

                    $crate::marker::Valid::raw(s)
                }
            }
        )*
    }
}

pub(crate) use gen_element;
