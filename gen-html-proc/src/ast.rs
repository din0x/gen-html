use std::collections::HashMap;

use crate::error::Error;
use proc_macro2::Span;
use syn::{Expr, Ident, Pat, spanned::Spanned};

pub struct Template {
    pub nodes: Vec<Node>,
}

pub enum Node {
    Element(Element),
    Block(Block),
    Expr(Expr),
    If(If),
    Match(Match),
    ForLoop(ForLoop),
}

impl Node {
    pub fn ends_with_eager_brace(&self) -> bool {
        matches!(
            self,
            Node::Block(_) | Node::Element(_) | Node::ForLoop(_) | Node::If(_) | Node::Match(_)
        )
    }
}

pub struct Element {
    pub name: Ident,
    pub attr_list: Vec<Attribute>,
    pub body: Option<Block>,
}

impl Element {
    pub fn validate(&self) -> Result<(), Error> {
        match crate::tags::is_self_closing(&self.name.to_string()) {
            Some(true) if self.body.is_some() => Err(Error::SelfClosing(self.name.clone())),
            Some(false) if self.body.is_none() => Err(Error::NotSelfClosing(self.name.clone())),
            Some(_) => Ok(()),
            None => Err(Error::InvalidTag(self.name.clone())),
        }
    }

    pub fn attributes(&self) -> Result<Vec<(String, Option<Expr>)>, Error> {
        let mut error = Error::empty();

        let mut key_to_spans = HashMap::new();
        for (span, key) in self.attr_list.iter().map(Attribute::key) {
            key_to_spans
                .entry(key.clone())
                .or_insert(Vec::new())
                .push(span);
        }

        for (key, spans) in key_to_spans
            .into_iter()
            .filter(|(_, spans)| spans.len() > 1)
        {
            error.push(Error::AttributeSpecifiedMoreThenOnce { spans, key });
        }

        if !error.is_empty() {
            return Err(error);
        }

        Ok(self
            .attr_list
            .iter()
            .map(|attr| (attr.key().1, attr.value().cloned()))
            .collect())
    }
}

pub enum Attribute {
    Id(Expr),
    Class(Expr),
    KeyValue { key: Ident, value: Option<Expr> },
}

impl Attribute {
    pub fn key(&self) -> (Span, String) {
        match self {
            Self::Id(id) => (id.span(), "id".to_owned()),
            Self::Class(class) => (class.span(), "class".to_owned()),
            Self::KeyValue { key, .. } => (
                key.span(),
                key.to_string().trim_start_matches("r#").replace("_", "-"),
            ),
        }
    }

    pub fn value(&self) -> Option<&Expr> {
        match self {
            Self::Id(id) => Some(id),
            Self::Class(class) => Some(class),
            Self::KeyValue { value, .. } => value.as_ref(),
        }
    }
}

pub struct Block {
    pub nodes: Vec<Node>,
}

pub struct If {
    pub cond: Expr,
    pub then_branch: Block,
    pub else_branch: Option<Box<Node>>,
}

pub struct Match {
    pub expr: Expr,
    pub arms: Vec<Arm>,
}

pub struct Arm {
    pub pat: Pat,
    pub guard: Option<Expr>,
    pub body: Node,
}

pub struct ForLoop {
    pub pat: Pat,
    pub expr: Expr,
    pub body: Block,
}
