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
/// # Syntax
///
/// Normal HTML elements are written with curly-braces.
///
/// ```
/// # let markup = gen_html::
/// html! {
///     p { "Lorem ipsum dolor sit amet." }
/// }
/// # ;
/// # assert_eq!(markup.to_string(), "<p>Lorem ipsum dolor sit amet.</p>");
/// ```
///
/// Self-closing tags require a trailing semicolon.
///
/// ```
/// # let markup = gen_html::
/// html! {
///     img src: "logo.svg" ;
///     // gets automatically changed to `data-cooldown`
///     button data_cooldown: "5s" onclick: "..." { "Click me" }
/// }
/// # ;
/// # assert_eq!(
/// #     markup.to_string(),
/// #     "<img src=\"logo.svg\"><button data-cooldown=\"5s\" onclick=\"...\">Click me</button>"
/// # );
/// ```
///
/// Using Rust expressions inside templates.
///
/// ```
/// # let name = "Bob";
/// # _ = gen_html::
/// html! {
///     p { (format!("Hello {name}")) }
///     img src: (format!("assets/{name}/profile-picture.png")) ;
/// }
/// # ;
/// ```
///
/// Conditional rendering with `if`.
///
/// ```
/// # let name = Some("Steve");
/// # _ = gen_html::
/// html! {
///     if let Some(name) = name {
///         (name)
///     } else {
///         "stranger"
///     }
/// }
/// # ;
/// ```
///
/// Pattern matching using `match`.
///
/// ```
/// # let age = 23;
/// # _ = gen_html::
/// html! {
///     match age {
///         ..18 => "You're not old enough",
///         18.. => p { "Hello world" }
///     }
/// }
/// # ;
/// ```
///
/// Repeating markup with `for`.
///
/// ```
/// # let titles = ["Titanic"];
/// # use gen_html::html;
/// # _ =
/// html! {
///     h1 { "Available movies" }
///     ul {
///         for title in titles {
///             li { (title) }
///         }
///     }
/// }
/// # ;
/// ```
pub use gen_html_proc::html;

pub use escape::Escaped;
pub use render::{Raw, Render, RenderFn, render_fn};
