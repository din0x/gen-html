use syn::{Expr, Ident, Pat};

use crate::error::{Error, ErrorKind};

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
            Some(true) if self.body.is_some() => Err(Error {
                span: self.name.span(),
                kind: ErrorKind::SelfClosing(self.name.to_string()),
            }),
            Some(false) if self.body.is_none() => Err(Error {
                span: self.name.span(),
                kind: ErrorKind::NotSelfClosing(self.name.to_string()),
            }),
            Some(_) => Ok(()),
            None => Err(Error {
                span: self.name.span(),
                kind: ErrorKind::InvalidTag(self.name.to_string()),
            }),
        }
    }
}

pub struct Attribute {
    pub name: Ident,
    pub value: Option<Expr>,
}

impl Attribute {
    pub fn name(&self) -> Result<String, Error> {
        let name_string = self.name.to_string();
        let name_string = name_string.trim_start_matches("r#");
        let is_valid = name_string
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch == '_')
            && !name_string.starts_with('_')
            && !name_string.ends_with('_')
            && !name_string.contains("__");

        is_valid
            .then(|| name_string.replace("_", "-"))
            .ok_or(Error::new(
                self.name.span(),
                ErrorKind::InvalidAttributeName(name_string.to_string()),
            ))
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
