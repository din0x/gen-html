# gen-html

<div>
    <a href="https://din0x.github.io/gen-html">
        <img height="20" src="https://img.shields.io/badge/docs-latest-blue?style=for-the-badge"/>
    </a>
    <a href="https://github.com/din0x/gen-html/actions/workflows/ci.yml">
        <img height="20" src="https://img.shields.io/github/actions/workflow/status/din0x/gen-html/ci.yml?style=for-the-badge">
    </a>
</div>

gen-html is a library for generating HTML from Rust code.

## Example
```rust 
use gen_html::{content::h1, element::{div, p}, inline_text::a};

let html = div((
    h1("This is a title"),
    p("Some paragraph"),
    p(("You can put <a> inside <p> ", a("click me").href("https://some-url.com")))
));
```
