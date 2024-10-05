use std::{fmt, marker::PhantomData};

/// String representing valid HTML.
#[derive(Eq)]
pub struct Valid<T> {
    html: String,
    _p: PhantomData<T>,
}

impl<T> fmt::Debug for Valid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<T, S: AsRef<str>> PartialEq<S> for Valid<T> {
    fn eq(&self, other: &S) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl Valid<Phrasing> {
    pub fn escaped(text: impl AsRef<str>) -> Self {
        let mut html = String::new();
        html_escape::encode_text_to_string(text, &mut html);

        Self::raw(html)
    }
}

impl Valid<Flow> {
    pub fn escaped(text: impl AsRef<str>) -> Self {
        let mut html = String::new();
        html_escape::encode_text_to_string(text, &mut html);

        Self::raw(html)
    }
}

impl<T> Valid<T> {
    pub fn new() -> Self {
        Self::raw(String::new())
    }

    pub(crate) fn raw(html: String) -> Self {
        Self {
            html,
            _p: PhantomData,
        }
    }
}

impl<T> AsRef<str> for Valid<T> {
    fn as_ref(&self) -> &str {
        &self.html
    }
}

impl<T> Element<T> for Valid<T> {
    fn into_html(self) -> Valid<T> {
        self
    }
}

impl<T> fmt::Display for Valid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

// impl<T, Iter: IntoIterator<Item = Valid<T>>> Element<T> for Iter {
//     default fn into_html(self) -> Valid<T> {
//         let mut s = String::new();

//         for elem in self {
//             s.push_str(elem.into_html().as_ref());
//         }

//         Valid::raw(s)
//     }
// }

macro_rules! category {
    (None) => {
        #[doc = concat!("Used with [`Valid<T>`] to mark elements that don't belong to any category.")]
        pub struct None;
    };
    ($name:ident) => {
        #[doc = concat!("Used with [`Valid<T>`] to mark an element as ", stringify!($name), " Content.")]
        pub struct $name;
    };
}

category!(Metadata);
category!(Flow);
category!(Phrasing);
category!(NotInteractive);
category!(None);

pub trait IntoElements
where
    Self: Sized,
{
    fn into_html(self) -> Elements<Self>;
}

impl<Iter: IntoIterator> IntoElements for Iter {
    fn into_html(self) -> Elements<Self> {
        Elements { iter: self }
    }
}

pub struct Elements<Iter> {
    iter: Iter,
}

impl<C, Item, Iter> Element<C> for Elements<Iter>
where
    Item: Element<C>,
    Iter: IntoIterator<Item = Item>,
{
    fn into_html(self) -> Valid<C> {
        let mut s = String::new();

        for elem in self.iter {
            s.push_str(elem.into_html().as_ref())
        }

        Valid::raw(s)
    }
}

pub trait Element<T>
where
    Self: Sized,
{
    fn into_html(self) -> Valid<T>;
}

impl Element<Phrasing> for &str {
    fn into_html(self) -> Valid<Phrasing> {
        Valid::<Phrasing>::escaped(self)
    }
}

impl Element<Flow> for &str {
    fn into_html(self) -> Valid<Flow> {
        Valid::<Flow>::escaped(self)
    }
}

macro_rules! impl_element_trait {
    ($t:ty, $group:ident, $body:expr) => {
        impl Element<$group> for $t {
            fn into_html(self) -> Valid<$group> {
                Valid::new()
            }
        }
    };
}

macro_rules! impl_element_trait_for_tuple {
    ($group:ident, [$($element:ident),*], [$($idx:tt),*]) => {
        impl<$($element: Element<$group>),*> Element<$group> for ($($element),*,) {
            fn into_html(self) -> Valid<$group> {
                let mut html = String::new();

            $(
                html.push_str((self).$idx.into_html().as_ref());
            )*

            Valid::raw(html)
            }
        }
    };
}

macro_rules! impl_tuple_elements {
    () => {
        impl_element_trait!((), Metadata, Valid::new());
        impl_element_trait!((), Flow, Valid::new());
        impl_element_trait!((), Phrasing, Valid::new());
    };
    ([$($element:ident),*], [$($idx:tt),*]) => {
        impl_element_trait_for_tuple!(Metadata, [$($element),*], [$($idx),*]);
        impl_element_trait_for_tuple!(Flow, [$($element),*], [$($idx),*]);
        impl_element_trait_for_tuple!(Phrasing, [$($element),*], [$($idx),*]);
    };
}

impl_tuple_elements!();
impl_tuple_elements!([T0], [0]);
impl_tuple_elements!([T0, T1], [0, 1]);
impl_tuple_elements!([T0, T1, T2], [0, 1, 2]);
impl_tuple_elements!([T0, T1, T2, T3], [0, 1, 2, 3]);
impl_tuple_elements!([T0, T1, T2, T3, T4], [0, 1, 2, 3, 4]);
impl_tuple_elements!([T0, T1, T2, T3, T4, T5], [0, 1, 2, 3, 4, 5]);
impl_tuple_elements!([T0, T1, T2, T3, T4, T5, T6], [0, 1, 2, 3, 4, 5, 6]);
impl_tuple_elements!([T0, T1, T2, T3, T4, T5, T6, T7], [0, 1, 2, 3, 4, 5, 6, 7]);
impl_tuple_elements!(
    [T0, T1, T2, T3, T4, T5, T6, T7, T8],
    [0, 1, 2, 3, 4, 5, 6, 7, 8]
);
impl_tuple_elements!(
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
);
impl_tuple_elements!(
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
);
impl_tuple_elements!(
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
);
impl_tuple_elements!(
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
);
impl_tuple_elements!(
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]
);
impl_tuple_elements!(
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
);
impl_tuple_elements!(
    [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
);

#[cfg(test)]
mod tests {
    use crate::html::P;

    use super::*;

    #[test]
    fn empty_tuple_belongs_to_every_content_category() {
        let elem = ();

        require::<Metadata>(elem);
        require::<Flow>(elem);
        require::<Phrasing>(elem);
    }

    #[test]
    fn tuple_belongs_to_every_content_category() {
        let elem = ((), P::new(""));

        require::<Flow>(elem);
    }

    #[test]
    fn p_is_flow_content() {
        require::<Flow>(P::new(""));
    }

    #[test]
    fn strings_are_escaped() {
        let s1 = Valid::<Phrasing>::escaped("&<>");
        let s2 = Valid::<Flow>::escaped("&<>");

        let expected = "&amp;&lt;&gt;";

        assert_eq!(s1, expected);
        assert_eq!(s2, expected);
    }

    fn require<T>(_e: impl Element<T>) {}
}
