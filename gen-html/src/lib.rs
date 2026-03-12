#![cfg_attr(docsrs, feature(doc_cfg))]

//! `gen-html` is a templating library for generating HTML from Rust.
//!
//! # Features
//!
//! - **Fast** — [`html!`] macro generates code that is as fast as writing to a string by hand.
//! - **Conditional rendering** — you can use `if`, `for` and `match` inside your templates.
//! - **Automatic escaping**, however you can opt-out using [`Raw<T>`].
//! - **Type safety** — HTML tags and attributes are checked at compile time.
//! - Integration with the rust web ecosystem (`axum`, `actix-web`).
//!
//! # Example
//!
//! ```
//! use gen_html::html;
//!
//! let markup = html! {
//!     for i in 1..=3 {
//!         span { (i.to_string()) }
//!     }
//! };
//!
//! println!("{}", markup);
//! # assert_eq!(markup.to_string(), "<span>1</span><span>2</span><span>3</span>");
//! ```
//!
//! The [`html!`] macro roughly expands to this code.
//!
//! ```
//! use gen_html::{Render, render_fn};
//!
//! let markup = render_fn(|f| {
//!     for i in 1..=3 {
//!         f.write_str("<span>")?;
//!         (&i.to_string()).render_to(f)?;
//!         f.write_str("</span>")?;
//!     }
//!     Ok(())
//! });
//!
//! /* ... */
//! # assert_eq!(markup.render().0, "<span>1</span><span>2</span><span>3</span>");
//! ```

mod escape;
mod render;
mod value;
mod web;

/// The `<!DOCTYPE html>` string literal.
///
/// # Example
///
/// ```
/// use gen_html::{DOCTYPE, html};
///
/// # let markup =
/// html! {
///     (DOCTYPE)
///     h1 { "hello world" }
/// }
/// # ;
/// # assert_eq!(markup.to_string(), "<!DOCTYPE html><h1>hello world</h1>");
/// ```
pub const DOCTYPE: Raw<&str> = Raw("<!DOCTYPE html>");

/// Generate HTML with `maud`-like syntax.
///
/// The `html!` macro allows you to write HTML using a `maud`-like syntax while
/// being as efficient as writing to a [`String`] by hand.
///
/// # HTML elements
///
/// Elements are written using their tag name followed by curly braces
/// containing their children.
///
/// ```
/// # use gen_html::html;
/// # let markup =
/// html! {
///     h1 { "Hello world" }
///     p { "This is a paragraph." }
/// }
/// # ;
///
/// # assert_eq!(
/// #     markup.to_string(),
/// #     "<h1>Hello world</h1><p>This is a paragraph.</p>"
/// # );
/// ```
///
/// Void elements (elements without closing tags) use a trailing semicolon.
///
/// ```
/// # use gen_html::html;
/// # let markup =
/// html! {
///     "First line"
///     br;
///     "Second line"
/// }
/// # ;
/// # assert_eq!(markup.to_string(), "First line<br>Second line");
/// ```
///
/// # Attributes
///
/// Attributes are written using `name: value`.
///
/// ```
/// # use gen_html::html;
/// # let markup =
/// html! {
///     a href: "https://example.com" { "Visit site" }
/// }
/// # ;
/// # assert_eq!(
/// #     markup.to_string(),
/// #     r#"<a href="https://example.com">Visit site</a>"#
/// # );
/// ```
///
/// Boolean attributes may omit the value.
///
/// ```
/// # use gen_html::html;
/// # let markup =
/// html! {
///     input r#type: "checkbox" checked;
/// }
/// # ;
/// # assert_eq!(
/// #     markup.to_string(),
/// #     r#"<input type="checkbox" checked>"#
/// # );
/// ```
///
/// # Shorthand syntax
///
/// Instead of writing `id` and `class` you may use `@` and `.` respectively.
///
/// - `@"container"` ==> `id: "container"`
/// - `."flex gap-2"` ==> `class: "flex gap-2"`
///
/// ```
/// # use gen_html::html;
/// # let markup =
/// html! {
///     div @"container" ."flex gap-2" { "content" }
/// }
/// # ;
/// # assert_eq!(
/// #     markup.to_string(),
/// #     r#"<div id="container" class="flex gap-2">content</div>"#
/// # );
/// ```
///
/// # Inserting expressions
///
/// Use `(expr)` to insert any Rust expression implementing [`Render`].
///
/// ```
/// # use gen_html::html;
/// let name = "Alice";
///
/// # let markup =
/// html! {
///     p { "Hello " (name) "!" }
/// }
/// # ;
/// # assert_eq!(markup.to_string(), "<p>Hello Alice!</p>");
/// ```
///
/// Expressions that implement [`Value`] may be used inside attributes. See its documentation for more details.
///
/// ```
/// # use gen_html::html;
/// let path = "/assets/logo.svg";
/// let description = "Company logo";
///
/// # let markup =
/// html! {
///     img src: (path) alt: (description);
/// }
/// # ;
/// # assert_eq!(
/// #     markup.to_string(),
/// #     r#"<img src="/assets/logo.svg" alt="Company logo">"#
/// # );
/// ```
///
/// # Control structures
///
/// ## `if`
///
/// ```
/// # use gen_html::html;
/// let logged_in = true;
///
/// # let markup =
/// html! {
///     if logged_in {
///         p { "Welcome back!" }
///     } else {
///         p { "Please log in." }
///     }
/// }
/// # ;
/// # assert_eq!(markup.to_string(), "<p>Welcome back!</p>");
/// ```
///
/// ## `if let`
///
/// ```
/// # use gen_html::html;
/// let name = Some("Damian");
///
/// # let markup =
/// html! {
///     if let Some(name) = name {
///         (name)
///     } else {
///        "stranger"
///     }
/// }
/// # ;
/// # assert_eq!(markup.to_string(), "Damian");
/// ```
///
/// ## `match`
///
/// ```
/// # use gen_html::html;
/// let status = 404;
///
/// # let markup =
/// html! {
///     match status {
///         200 => p { "OK" },
///         404 => p { "Not found" },
///         _ => p { "Unknown status" }
///     }
/// }
/// # ;
/// # assert_eq!(markup.to_string(), "<p>Not found</p>");
/// ```
///
/// ## `for`
///
/// Use `for` loops to render repeated elements from an iterator.
///
/// ```
/// # use gen_html::html;
/// let shows = ["Breaking Bad", "Planet Earth II", "Chernobyl"];
///
/// # let markup =
/// html! {
///     h1 { "Popular TV shows" }
///     ul {
///         for title in shows {
///             li { (title) }
///         }
///     }
/// }
/// # ;
/// # assert_eq!(
/// #     markup.to_string(),
/// #     "<h1>Popular TV shows</h1><ul><li>Breaking Bad</li><li>Planet Earth II</li><li>Chernobyl</li></ul>"
/// # );
/// ```
pub use gen_html_proc::html;

pub use escape::Escaped;
pub use render::{Raw, Render, RenderFn, render_fn};
pub use value::Value;
