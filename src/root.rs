use std::borrow::Cow;

use crate::{element::elem, FlowContent, Render};

elem!("", Body, body, FlowContent,);
elem!("", Head, head, Render,);
elem!("", HtmlBase, html, Render,);

elem!("", Title, title, Render,);

pub struct Page<T: FlowContent> {
    title: Cow<'static, str>,
    body: T,
}

impl<T: FlowContent> Render for Page<T> {
    fn render(self, target: &mut dyn std::fmt::Write) -> std::fmt::Result {
        
        html((head(title(self.title)), body(self.body))).render(target)
    }
}

pub fn page<Title, Content>(title: Title, body: Content) -> Page<Content>
where
    Title: Into<Cow<'static, str>>,
    Content: FlowContent,
{
    Page {
        body,
        title: title.into(),
    }
}
