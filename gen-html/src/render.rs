use crate::escape::escape;
use std::{
    borrow::Cow,
    fmt::{self, Arguments},
};

/// Trait for safely rendering HTML content.
///
/// `Render` is similar to [`Display`] and [`Debug`] but it is for safely generating HTML.
///
/// # Example
///
/// ```
/// use std::fmt::{Formatter, Result};
/// use gen_html::Render;
///
/// struct Vector2D {
///     x: f32,
///     y: f32,
/// }
///
/// impl Render for Vector2D {
///     fn render_to(&self, f: &mut Formatter) -> Result {
///         let Self { x, y } = self;
///         // `f32` cannot contain special characters, so we don't need to worry about escaping
///         write!(f, "({x}, {y})")
///     }
/// }
/// # let point = Vector2D { x: 2.0, y: -1.0 };
/// # assert_eq!(point.render().0, "(2, -1)");
/// ```
///
/// [`Display`]: std::fmt::Display
/// [`Debug`]: std::fmt::Debug
/// [`Escaped`]: crate::Escaped
#[diagnostic::on_unimplemented(
    message = "the type `{Self}` cannot be safely rendered as HTML",
    note = "to safely render `{Self}` as HTML, implement the `Render` trait or wrap it in `Escaped<{Self}>`"
)]
pub trait Render {
    /// Renders HTML to a given formatter.
    ///
    /// When implementing this function, you should make sure that the output is valid HTML.
    ///
    /// # Errors
    ///
    /// This function should return [`Err`] if, and only if, the provided [`Formatter`] returns [`Err`].
    ///
    /// [`Formatter`]: std::fmt::Formatter
    fn render_to(&self, f: &mut fmt::Formatter) -> fmt::Result;

    /// Converts the given value to a `Raw<String>`.
    ///
    /// ```
    /// # use gen_html::Render;
    /// let content = "<this is escaped>";
    /// assert_eq!(content.render().0, "&lt;this is escaped&gt;");
    /// ```
    fn render(&self) -> Raw<String> {
        use std::fmt::Write;
        let mut buf = String::new();
        write!(&mut buf, "{}", render_fn(|f| self.render_to(f))).expect("render_to returned Err");
        Raw(buf)
    }
}

impl Render for str {
    fn render_to(&self, f: &mut fmt::Formatter) -> fmt::Result {
        escape(self, f)
    }
}

impl Render for String {
    fn render_to(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().render_to(f)
    }
}

impl Render for Arguments<'_> {
    fn render_to(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.as_str() {
            Some(s) => s.render_to(f),
            None => self.to_string().render_to(f),
        }
    }
}

impl<B> Render for Cow<'_, B>
where
    B: Render + ToOwned + ?Sized,
{
    fn render_to(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ref().render_to(f)
    }
}

macro_rules! ref_render_impl {
    ( $( $t:ty )* ) => {
        $(
            impl<T> Render for $t
            where
                T: Render + ?Sized,
            {
                fn render_to(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    T::render_to(self, f)
                }
            }
        )*
    };
}

ref_render_impl! {
    &T
    &mut T
    Box<T>
    std::rc::Rc<T>
    std::sync::Arc<T>
}

macro_rules! trusted_render_impl {
    ( $( $t:ty )* ) => {
        $(
            impl Render for $t {
                fn render_to(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "{self}")
                }
            }
        )*
    };
}

trusted_render_impl! {
    f32 f64
    i8 i16 i32 i64 i128 isize
    u8 u16 u32 u64 u128 usize
}

/// Wrapper to render content using [`Display`] without escaping.
///
/// Use this wrapper when you have HTML content that is already safe and should be
/// rendered without escaping. This should generally only be used for trusted content,
/// not for user-provided input.
///
/// # Example
///
/// ```
/// use gen_html::{html, Raw};
///
/// let oopsie = html! {
///     p { (Raw("<div>")) }
/// };
/// # assert_eq!(oopsie.to_string(), "<p><div></p>");
/// ```
///
/// [`Display`]: std::fmt::Display
#[derive(Debug, Clone, Copy)]
pub struct Raw<T>(pub T);

impl<T: fmt::Display> fmt::Display for Raw<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::Display> Render for Raw<T> {
    fn render_to(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Implements [`Render`] using a function.
///
/// This `struct` is created by [`render_fn()`].
pub struct RenderFn<F> {
    f: F,
}

impl<F> fmt::Debug for RenderFn<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RenderFn").finish()
    }
}

impl<F> Render for RenderFn<F>
where
    F: Fn(&mut fmt::Formatter) -> fmt::Result,
{
    fn render_to(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self.f)(f)
    }
}

impl<F> fmt::Display for RenderFn<F>
where
    F: Fn(&mut fmt::Formatter) -> fmt::Result,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self.f)(f)
    }
}

/// Creates a type whose [`Render`] impl is provided with the function `f`.
pub fn render_fn<F>(f: F) -> RenderFn<F>
where
    F: Fn(&mut fmt::Formatter) -> fmt::Result,
{
    RenderFn { f }
}
