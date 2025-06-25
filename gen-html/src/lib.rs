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
/// ## Elements
///
/// Normal HTML elements are written with curly-braces.
///
/// ```
/// # let markup = gen_html::
/// html! {
///     h1 { "Lorem ipsum" }
///     p { "Lorem ipsum dolor sit amet." }
/// }
/// # ; assert_eq!(markup.to_string(), "<h1>Lorem ipsum</h1><p>Lorem ipsum dolor sit amet.</p>");
/// ```
///
/// Void elements require a trailing semicolon.
///
/// ```
/// # let markup = gen_html::
/// html! {
///     p {
///         "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
///         br ;
///         "Praesent pharetra urna ex."
///     }
/// }
/// # ; assert_eq!(
/// #     markup.to_string(),
/// #     "<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit.<br>Praesent pharetra urna ex.</p>",
/// # );
/// ```
///
/// ## Attributes
///
/// Add an attribute with `attr_name: "value"` syntax.
///
/// ```
/// # let markup = gen_html::
/// html! {
///     img src: "logo.svg" ;
///     // gets automatically changed to `data-cooldown`
///     button data_cooldown: "5s" onclick: "..." { "Click me" }
/// }
/// # ; assert_eq!(
/// #     markup.to_string(),
/// #     r#"<img src="logo.svg"><button data-cooldown="5s" onclick="...">Click me</button>"#,
/// # );
/// ```
///
/// You may omit the value to create an empty attribute.
///
/// ```
/// # let markup = gen_html::
/// html! {
///     p { "I agree to the terms of service." }
///     input r#type: "checkbox" checked ;
/// }
/// # ; assert_eq!(markup.to_string(), r#"<p>I agree to the terms of service.</p><input type="checkbox" checked>"#);
/// ```
///
/// Specifying the `id` and `class` can be done using the attribute syntax or their respective
/// shorthand `@` and `.`. The `@` character was chosen due to rust syntax using `#""#` for string
/// literals.
///
/// ```
/// # let markup = gen_html::
/// html! {
///     div id: "my-div-1" class: "flex flex-col gap-2" { "..." }
///     div @"my-div-2" ."flex flex-col gap-2" { "..." }
/// }
/// # ; assert_eq!(
/// #     markup.to_string(),
/// #     r#"<div id="my-div-1" class="flex flex-col gap-2">...</div><div id="my-div-2" class="flex flex-col gap-2">...</div>"#,
/// # );
/// ```
///
/// ## Inserting expressions
///
/// Insert arbitrary expression of type that implements the [`Render`] trait with `(expr)` syntax.
///
/// ```
/// # let name = "Bob";
/// # let class_list = "class-1 class-2";
/// # let id = "my-id";
/// # let markup = gen_html::
/// html! {
///     p {
///         "Hello " (name)
///     }
///     img src: (format!("assets/{name}/profile-picture.png")) ;
///     // Also with the shorthand syntax
///     div @(id) .(class_list) {}
/// }
/// # ; assert_eq!(
/// #     markup.to_string(),
/// #     r#"<p>Hello Bob</p><img src="assets/Bob/profile-picture.png"><div id="my-id" class="class-1 class-2"></div>"#,
/// # );
/// ```
///
/// ## Control structures
///
/// Conditional rendering with `if`.
///
/// ```
/// # let age = 19;
/// # let markup = gen_html::
/// html! {
///     if age < 18 {
///         p { "You're not old enough" }
///     } else if age == 18 {
///         p { "Just in time" }
///     } else {
///         p { "Hi" }
///     }
/// }
/// # ; assert_eq!(markup.to_string(), "<p>Hi</p>");
/// ```
///
/// `if let` is also supported.
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
/// Use `for` to loop over elements of an iterator.
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
