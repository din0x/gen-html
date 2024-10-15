use std::borrow::Cow;

use crate::{element::elem, Flow, Html};

elem!("", Body, body, Flow,);
elem!("", Head, head, Html,);
elem!("", HtmlBase, html, Html,);

elem!("", Title, title, Html,);

pub struct Page<T: Flow> {
    title: Cow<'static, str>,
    #[cfg(feature = "css-style")]
    style: Option<crate::css_style::Style>,
    body: T,
}

impl<T: Flow> Page<T> {
    #[cfg(feature = "css-style")]
    pub fn style(mut self, style: crate::css_style::Style) -> Self {
        self.style = Some(style);
        self
    }
}

impl<T: Flow> Html for Page<T> {
    fn fmt_html(self, target: &mut dyn std::fmt::Write) -> std::fmt::Result {
        #[cfg(not(feature = "css-style"))]
        return html((head(title(self.title)), body(self.body))).fmt_html(target);
        #[cfg(feature = "css-style")]
        {
            let mut body = body(self.body);

            if let Some(style) = self.style {
                body = body.style(style);
            }

            html((head(title(self.title)), body)).fmt_html(target)
        }
    }
}

pub fn page<Title, Content>(title: Title, body: Content) -> Page<Content>
where
    Title: Into<Cow<'static, str>>,
    Content: Flow,
{
    Page {
        body,
        #[cfg(feature = "css-style")]
        style: None,
        title: title.into(),
    }
}
