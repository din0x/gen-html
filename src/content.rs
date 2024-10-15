//! Content sectioning elements.
//!
//! Content sectioning elements allow you to organize the document content into logical pieces.
//! Use the sectioning elements to create a broad outline for your page content, including header
//! and footer navigation, and heading elements to identify sections of content.
//!
//! [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element#content_sectioning)

use crate::{element::flow_elem, Flow, Phrasing};

flow_elem!(pub Address, address, Flow);
flow_elem!(pub Article, article, Flow);
flow_elem!(pub Aside, aside, Flow);
flow_elem!(pub Footer, footer, Flow);
flow_elem!(pub Header, header, Flow);

flow_elem!(pub H1, h1, Phrasing);
flow_elem!(pub H2, h2, Phrasing);
flow_elem!(pub H3, h3, Phrasing);
flow_elem!(pub H4, h4, Phrasing);
flow_elem!(pub H5, h5, Phrasing);
flow_elem!(pub H6, h6, Phrasing);

// TODO: `hgroup`

flow_elem!(pub Main, main, Flow);
flow_elem!(pub Nav, nav, Flow);
flow_elem!(pub Section, section, Flow);
flow_elem!(pub Search, search, Flow);
