//! Content sectioning elements.
//!
//! Content sectioning elements allow you to organize the document content into logical pieces.
//! Use the sectioning elements to create a broad outline for your page content, including header
//! and footer navigation, and heading elements to identify sections of content.
//!
//! [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element#content_sectioning)

use crate::{element::flow_elem, FlowContent, PhrasingContent};

flow_elem!(pub Address, address, FlowContent, content);
flow_elem!(pub Article, article, FlowContent, content);
flow_elem!(pub Aside, aside, FlowContent, content);
flow_elem!(pub Footer, footer, FlowContent, content);
flow_elem!(pub Header, header, FlowContent, content);

flow_elem!(pub H1, h1, PhrasingContent, content);
flow_elem!(pub H2, h2, PhrasingContent, content);
flow_elem!(pub H3, h3, PhrasingContent, content);
flow_elem!(pub H4, h4, PhrasingContent, content);
flow_elem!(pub H5, h5, PhrasingContent, content);
flow_elem!(pub H6, h6, PhrasingContent, content);

// TODO: `hgroup`

flow_elem!(pub Main, main, FlowContent, content);
flow_elem!(pub Nav, nav, FlowContent, content);
flow_elem!(pub Section, section, FlowContent, content);
flow_elem!(pub Search, search, FlowContent, content);
