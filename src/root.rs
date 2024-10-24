use std::borrow::Cow;

use crate::{element::elem, FlowContent, Render};

elem!("", Body, body, FlowContent,);
elem!("", Head, head, Render,);
elem!("", HtmlBase, html, Render,);

elem!("", Title, title, Render,);

pub struct Page<T: FlowContent> {
    title: Cow<'static, str>,
    #[cfg(feature = "css-style")]
    style: Option<crate::css_style::Style>,
    body: T,
}

impl<T: FlowContent> Page<T> {
    #[cfg(feature = "css-style")]
    pub fn style(mut self, style: crate::css_style::Style) -> Self {
        self.style = Some(style);
        self
    }
}

impl<T: FlowContent> Render for Page<T> {
    fn render(self, target: &mut dyn std::fmt::Write) -> std::fmt::Result {
        #[cfg(not(feature = "css-style"))]
        return html((head(title(self.title)), body(self.body))).render(target);
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
    Content: FlowContent,
{
    Page {
        body,
        #[cfg(feature = "css-style")]
        style: None,
        title: title.into(),
    }
}
