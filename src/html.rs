use crate::{gen::gen_element, Flow, NotInteractive, Phrasing};

macro_rules! gen_flow {
    ($name:ident, $tag:ident, [$($types:ty),*]) => {
        gen_element!($name, $tag, Flow, concat!("```\nuse gen_html::html::*;\n\nlet html: Valid<Flow> = ", stringify!($tag), "(\"Some text\").into_html();\nassert_eq!(html, \"<", stringify!($tag), ">Some text</", stringify!($tag), ">\");"), [$($types),*]);
    };
}

macro_rules! gen_phrasing {
    ($name:ident, $tag:ident, [$($types:ty),*]) => {
        gen_element!($name, $tag, Phrasing, concat!("```\nuse gen_html::html::*;\n\nlet html: Valid<Phrasing> = ", stringify!($tag), "(\"Some text\").into_html();\nassert_eq!(html, \"<", stringify!($tag), ">Some text</", stringify!($tag), ">\");"), [$($types),*]);
    };
}

gen_flow!(Div, div, [Flow]);

gen_phrasing!(P, p, [Phrasing]);

gen_phrasing!(A, a, [NotInteractive]);

gen_phrasing!(H1, h1, [Phrasing]);
gen_phrasing!(H2, h2, [Phrasing]);
gen_phrasing!(H3, h3, [Phrasing]);
gen_phrasing!(H4, h4, [Phrasing]);
gen_phrasing!(H5, h5, [Phrasing]);
gen_phrasing!(H6, h6, [Phrasing]);
