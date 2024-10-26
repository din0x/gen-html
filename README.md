# gen-html

[<img height="20" src="https://img.shields.io/badge/docs-latest-blue?style=for-the-badge"/>](https://din0x.github.io/gen-html)
[<img height="20" src="https://img.shields.io/github/actions/workflow/status/din0x/gen-html/ci.yml?style=for-the-badge">](https://github.com/din0x/gen-html/actions/workflows/ci.yml)

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
