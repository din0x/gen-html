# `gen-html`

[<img height="25" src="https://img.shields.io/badge/github-8da0cb?style=for-the-badge&logo=github">](https://github.com/din0x/gen-html/)
[<img height="25" src="https://img.shields.io/crates/v/gen-html?style=for-the-badge&logo=rust">](https://crates.io/crates/gen-html/)
[<img height="25" src="https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&logo=docs.rs&labelColor=555555">](https://docs.rs/gen-html/latest/gen_html/)
[<img height="25" src="https://img.shields.io/github/actions/workflow/status/din0x/gen-html/ci.yml?style=for-the-badge">](https://github.com/din0x/gen-html/actions/workflows/ci.yml)

`gen-html` is a library for generating HTML from Rust code.

## Example
```rust 
use gen_html::{content::h1, inline_text::a, text_content::{div, p}};

let html = div((
    h1("This is a title"),
    p("Some paragraph"),
    p(("You can put <a> inside <p> ", a("click me").href("https://some-url.com")))
));
```
