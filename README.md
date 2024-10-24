# gen-html

The goal of this library is to provide a simple way to generate HTML elements
from rust code.

## Example
```rust 
use gen_html::{content::h1, element::{div, p}, inline_text::a};

let html = div((
    h1("This is a title"),
    p("Some paragraph"),
    p(("You can put <a> inside <p> ", a("click me").href("https://some-url.com")))
));
```
