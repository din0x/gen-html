# gen-html

The goal of this library is to provide a simple way to generate HTML elements
from rust code.

## Example
```rust 
use gen_html::element::{div, h1, p, a};

let html = div((
    h1("This is a title"),
    p("Some paragraph"),
    p(("You can put <a> inside <p> ", a("click me", "https://some-url.com")))
));
```
