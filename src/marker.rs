use std::{borrow::Cow, fmt};

pub trait Html: Sized {
    fn fmt_html(self, target: &mut dyn fmt::Write) -> fmt::Result;

    fn to_html_string(self) -> String {
        let mut s = String::new();
        self.fmt_html(&mut s)
            .expect("writing to `String` should not fail");
        s
    }
}

pub trait Flow: Html {}

pub trait Phrasing: Flow {}

pub trait NonInteractive: Html {}

impl Html for &str {
    fn fmt_html(self, target: &mut dyn fmt::Write) -> fmt::Result {
        write!(target, "{}", html_escape::encode_text(self))
    }
}

impl Flow for &str {}

impl Phrasing for &str {}

impl NonInteractive for &str {}

impl Html for String {
    fn fmt_html(self, target: &mut dyn fmt::Write) -> fmt::Result {
        write!(target, "{}", html_escape::encode_text(&self))
    }
}

impl Flow for String {}

impl Phrasing for String {}

impl NonInteractive for String {}

impl<'a> Html for Cow<'a, str> {
    fn fmt_html(self, target: &mut dyn fmt::Write) -> fmt::Result {
        self.as_ref().fmt_html(target)
    }
}

impl<'a> Flow for Cow<'a, str> {}

impl<'a> Phrasing for Cow<'a, str> {}

impl<'a> NonInteractive for Cow<'a, str> {}

macro_rules! impl_tuple {
    () => {
        impl Html for () {
            fn fmt_html(self, _: &mut dyn fmt::Write) -> fmt::Result {
                Ok(())
            }
        }

        impl Flow for () {}
        impl Phrasing for () {}
        impl NonInteractive for () {}
    };
    ($($t:ident),*) => {
        impl<$($t:Html),*> Html for ($($t),*,) {
            fn fmt_html(self, target: &mut dyn fmt::Write) -> fmt::Result {
                $(
                    self.${index()}.fmt_html(target)?;
                    ${ignore($t)}
                )*
                Ok(())
            }
        }

        impl<$($t:Flow),*> Flow for ($($t),*,) {}
        impl<$($t:Phrasing),*> Phrasing for ($($t),*,) {}
        impl<$($t:NonInteractive),*> NonInteractive for ($($t),*,) {}
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
