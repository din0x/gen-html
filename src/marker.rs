use std::{borrow::Cow, fmt};

pub trait Render: Sized {
    fn render(self, target: &mut dyn fmt::Write) -> fmt::Result;

    fn render_to_string(self) -> String {
        let mut s = String::new();
        self.render(&mut s)
            .expect("writing to `String` should not fail");
        s
    }
}

/// The Flow content category.
///  
/// [Flow content] is a broad category that encompasses most elements that can go
/// inside the `<body>` element, including heading elements, sectioning elements,
/// phrasing elements, embedding elements, interactive elements, and
/// form-related elements. It also includes text nodes (but not those that only
/// consist of white space characters).
///
/// [Flow content]: https://developer.mozilla.org/en-US/docs/Web/HTML/Content_categories#flow_content
pub trait FlowContent: Render {}

/// The Phrasing content category.
///
/// [Phrasing content], a subset of flow content, refers to the text and the
/// markup within a document. Sequences of phrasing content make up paragraphs.
///
/// [Phrasing content]: https://developer.mozilla.org/en-US/docs/Web/HTML/Content_categories#phrasing_content
pub trait PhrasingContent: FlowContent {}

/// Non-interactive content category.
///
/// [Interactive content], a subset of flow content, includes elements that are
/// specifically designed for user interaction.
///
/// [Interactive content]: https://developer.mozilla.org/en-US/docs/Web/HTML/Content_categories#interactive_content
pub trait NonInteractiveContent: Render {}

impl Render for &str {
    fn render(self, target: &mut dyn fmt::Write) -> fmt::Result {
        write!(target, "{}", html_escape::encode_text(self))
    }
}

impl FlowContent for &str {}

impl PhrasingContent for &str {}

impl NonInteractiveContent for &str {}

impl Render for String {
    fn render(self, target: &mut dyn fmt::Write) -> fmt::Result {
        write!(target, "{}", html_escape::encode_text(&self))
    }
}

impl FlowContent for String {}

impl PhrasingContent for String {}

impl NonInteractiveContent for String {}

impl<'a> Render for Cow<'a, str> {
    fn render(self, target: &mut dyn fmt::Write) -> fmt::Result {
        self.as_ref().render(target)
    }
}

impl<'a> FlowContent for Cow<'a, str> {}

impl<'a> PhrasingContent for Cow<'a, str> {}

impl<'a> NonInteractiveContent for Cow<'a, str> {}

macro_rules! impl_tuple {
    () => {
        impl Render for () {
            fn render(self, _: &mut dyn fmt::Write) -> fmt::Result {
                Ok(())
            }
        }

        impl FlowContent for () {}
        impl PhrasingContent for () {}
        impl NonInteractiveContent for () {}
    };
    ($($t:ident),*) => {
        impl<$($t:Render),*> Render for ($($t),*,) {
            fn render(self, target: &mut dyn fmt::Write) -> fmt::Result {
                $(
                    self.${index()}.render(target)?;
                    ${ignore($t)}
                )*
                Ok(())
            }
        }

        impl<$($t:FlowContent),*> FlowContent for ($($t),*,) {}
        impl<$($t:PhrasingContent),*> PhrasingContent for ($($t),*,) {}
        impl<$($t:NonInteractiveContent),*> NonInteractiveContent for ($($t),*,) {}
    };
}

impl_tuple!();
impl_tuple!(T0);
impl_tuple!(T0, T1);
impl_tuple!(T0, T1, T2);
impl_tuple!(T0, T1, T2, T3);
impl_tuple!(T0, T1, T2, T3, T4);
impl_tuple!(T0, T1, T2, T3, T4, T5);
impl_tuple!(T0, T1, T2, T3, T4, T5, T6);
impl_tuple!(T0, T1, T2, T3, T4, T5, T6, T7);
impl_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
impl_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
