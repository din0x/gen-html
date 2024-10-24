macro_rules! create_element {
    ($doc:expr, $vis:vis ($name:ident, $tag:ident): [$( $content:path ),*], [
        $(
            {
                $attr:ident, $display:ident,
                $val:ty,
                {$( $t:tt )*}
            }
        ),*
    ]) => {
        #[derive(Default)]
        #[doc = concat!("Builder for the `<", stringify!($tag), ">` HTML element.\n\n")]
        #[doc = $doc]
        $vis struct $name<T: $($content+)*> {
            content: T,
            id: Option<$crate::attribute::Id>,
            class: $crate::attribute::ClassList,
            lang: Option<$crate::attribute::Lang>,
            $(
                $attr: $val,
            )*
        }

        impl<T: $( $content + )*> $name<T> {
            /// Sets the `id` attribute.
            pub fn id(mut self, id: $crate::attribute::Id) -> Self {
                self.id = Some(id);
                self
            }

            /// Sets the `class` attribute.
            pub fn class(mut self, class: impl Into<$crate::attribute::ClassList>) -> Self {
                self.class = class.into();
                self
            }

            /// Sets the `lang` attribute.
            pub fn lang(mut self, lang: $crate::attribute::Lang) -> Self {
                self.lang = Some(lang);
                self
            }

            $(
                $( $t )*
            )*
        }

        impl<T: $crate::Render + $( $content + )*> $crate::Render for $name<T> {
            fn render(self, t: &mut dyn ::std::fmt::Write) -> ::std::fmt::Result {
                write!(t, concat!("<", stringify!($tag)))?;

                use $crate::attribute::Attribute;

                self.id.render_attr("id", t)?;
                self.class.render_attr("class", t)?;
                self.lang.render_attr("lang", t)?;

                $(
                    self.$attr.render_attr(stringify!($display), t)?;
                )*

                write!(t, ">")?;
                self.content.render(t)?;
                write!(t, concat!("</", stringify!($tag), ">"))?;

                Ok(())
            }
        }

        #[doc = concat!("The `<", stringify!($tag), ">` HTML element.\n\n")]
        #[doc = $doc]
        $vis fn $tag<T: $( $content + )*>(content: T) -> $name<T> {
            $name {
                content,
                ..$name::<()>::default()
            }
        }
    };
}

pub(crate) use create_element;
