# gen-html

The goal of this library is to provide a simple way to generate HTML elements
from rust code.

## Examples
```rust
use gen_html::prelude::*;

let html = Div::with((
    H1::with("Hello world"),
    P::with("This is a paragraph"),
    P::with(("This <p> contains <a> tag", A::new("click me"))),
))
.class("content");

let expected = concat!(
    "<div class=\"content\">",
        "<h1>Hello world</h1>",
        "<p>This is a paragraph</p>",
        "<p>This &lt;p&gt; contains &lt;a&gt; tag<a>click me</a></p>",
    "</div>",
);

assert_eq!(html.into_html(), expected);
```

## Goal
```
use gen_html;

let html = div((
    h1("Hello world"),
    p("This is a paragraph"),
    p(("This <p> contains <a> tag", a("click me", "https://some-url.com")))
));
```
