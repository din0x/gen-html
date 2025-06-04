# `gen-html`

[<img height="25" src="https://img.shields.io/crates/v/gen-html?style=for-the-badge&logo=rust">](https://crates.io/crates/gen-html/)
[<img height="25" src="https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&logo=docs.rs&labelColor=555555">](https://docs.rs/gen-html/latest/gen_html/)
[<img height="25" src="https://img.shields.io/github/actions/workflow/status/din0x/gen-html/ci.yml?style=for-the-badge">](https://github.com/din0x/gen-html/actions/workflows/ci.yml)

`gen-html` is a templating library for generating HTML from Rust.

## Features

 - **Fast** — `html!` macro generates code that is as fast as writing to a string by hand.
 - **Conditional rendering** — you can use `if`, `for` and `match` inside your templates.
 - **Automatic escaping**, however you can opt-out using `Raw<T>`.
 - **Type safety** — HTML tags and attributes are checked at compile time.
 - Integration with the rust web ecosystem (`axum`, `actix-web`).

## Example

```rust
use gen_html::html;

let markup = html! {
    for i in 1..=3 {
        span { (i.to_string()) }
    }
};

println!("{}", markup);
```

The `html!` macro roughly expands to this code.

```rust
use gen_html::{Render, render_fn};

let markup = render_fn(|f| {
    for i in 1..=3 {
        f.write_str("<span>")?;
        (&i.to_string()).render_to(f)?;
        f.write_str("</span>")?;
    }
    Ok(())
});

/* ... */
```
