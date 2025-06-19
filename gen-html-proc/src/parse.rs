use crate::ast::{Arm, Attribute, Block, Element, ForLoop, If, Match, Node, Template};
use syn::{
    Expr, ExprLit, Ident, Lit, LitStr, Pat, Token, braced, parenthesized,
    parse::{self, Parse},
    token,
};

impl Parse for Template {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut nodes = Vec::new();

        while !input.is_empty() {
            nodes.push(Node::parse(input)?);
        }

        Ok(Self { nodes })
    }
}

impl Parse for Node {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Token![if]) {
            If::parse(input).map(Self::If)
        } else if input.peek(Token![match]) {
            Match::parse(input).map(Self::Match)
        } else if input.peek(Token![for]) {
            ForLoop::parse(input).map(Self::ForLoop)
        } else if input.peek(token::Paren) {
            let content;
            parenthesized!(content in input);
            Expr::parse(&content).map(Self::Expr)
        } else if input.peek(token::Brace) {
            Block::parse(input).map(Self::Block)
        } else if let Ok(s) = input.parse::<LitStr>() {
            Ok(Self::Expr(Expr::Lit(ExprLit {
                attrs: Vec::new(),
                lit: Lit::Str(s),
            })))
        } else {
            Element::parse(input).map(Self::Element)
        }
    }
}

impl Parse for Element {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = Ident::parse(input)?;
        let mut attr_list = Vec::new();

        while input.peek(Ident) || input.peek(Token![@]) || input.peek(Token![.]) {
            attr_list.push(Attribute::parse(input)?);
        }

        let body = if input.peek(Token![;]) {
            <Token![;]>::parse(input)?;
            None
        } else if input.peek(token::Brace) {
            Some(Block::parse(input)?)
        } else {
            Err(input.error("expected curly braces or SEMICOLON"))?
        };

        Ok(Self {
            name,
            attr_list,
            body,
        })
    }
}

impl Parse for Attribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if <Token![@]>::parse(input).is_ok() {
            let id = parse_attribute_value(input)?;
            Ok(Self::Id(id))
        } else if <Token![.]>::parse(input).is_ok() {
            let class = parse_attribute_value(input)?;
            Ok(Self::Class(class))
        } else if let Ok(key) = Ident::parse(input) {
            let value = if <Token![:]>::parse(input).is_ok() {
                Some(parse_attribute_value(input)?)
            } else {
                None
            };
            Ok(Self::KeyValue { key, value })
        } else {
            Err(syn::Error::new(input.span(), "todo"))
        }
    }
}

impl Parse for Block {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let content;
        braced!(content in input);

        let mut nodes = Vec::new();

        while !content.is_empty() {
            nodes.push(Node::parse(&content)?);
        }

        Ok(Self { nodes })
    }
}

impl Parse for If {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        <Token![if]>::parse(input)?;
        let cond = Expr::parse_without_eager_brace(input)?;
        let then_branch = Block::parse(input)?;
        let else_branch = if <Token![else]>::parse(input).is_ok() {
            if input.peek(Token![if]) {
                <Token![else]>::parse(input)?;
                Some(Box::new(Node::If(Self::parse(input)?)))
            } else {
                Some(Box::new(Node::Block(Block::parse(input)?)))
            }
        } else {
            None
        };

        Ok(Self {
            cond,
            then_branch,
            else_branch,
        })
    }
}

impl Parse for Match {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![match]>()?;
        let expr = Expr::parse_without_eager_brace(input)?;
        let content;
        braced!(content in input);

        let mut arms = Vec::new();
        while !content.is_empty() {
            arms.push(Arm::parse(&content)?);
        }

        Ok(Self { expr, arms })
    }
}

impl Parse for Arm {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let pat = Pat::parse_multi_with_leading_vert(input)?;

        let guard = input
            .parse::<Token![if]>()
            .map(|_| Expr::parse(input))
            .ok()
            .transpose()?;

        <Token![=>]>::parse(input)?;

        let body = Node::parse(input)?;

        if body.ends_with_eager_brace() || input.is_empty() {
            _ = <Token![,]>::parse(input)
        } else {
            <Token![,]>::parse(input)?;
        }

        Ok(Self { pat, guard, body })
    }
}

impl Parse for ForLoop {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        <Token![for]>::parse(input)?;
        let pat = Pat::parse_multi_with_leading_vert(input)?;
        <Token![in]>::parse(input)?;
        let expr = Expr::parse_without_eager_brace(input)?;
        let body = Block::parse(input)?;

        Ok(Self { pat, expr, body })
    }
}

fn parse_attribute_value(input: syn::parse::ParseStream) -> syn::Result<Expr> {
    <LitStr as Parse>::parse(input)
        .map(|s| {
            Expr::Lit(ExprLit {
                attrs: Vec::new(),
                lit: Lit::Str(s),
            })
        })
        .or_else(|_| {
            let content;
            parenthesized!(content in input);
            Expr::parse(&content)
        })
}
