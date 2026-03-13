use crate::{
    ast::{Block, Element, ForLoop, If, Let, Match, Node, Template},
    error::Error,
};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    Expr, ExprBlock, ExprForLoop, ExprIf, ExprMatch, Ident, Local, LocalInit, Stmt, Token,
    parse_quote, spanned::Spanned, token::Brace,
};

impl Template {
    pub fn expand(self) -> TokenStream {
        let output_ident = Ident::new("__f", Span::call_site());
        let block = self.finish(output_ident.clone());

        quote! {
            ::gen_html::render_fn(move |#output_ident: &mut ::std::fmt::Formatter| {
                #block
                ::std::fmt::Result::Ok(())
            })
        }
    }
}

trait Build {
    fn generate(&self, ctx: &mut Context);

    fn finish(&self, output_ident: Ident) -> syn::Block {
        let mut ctx = Context::new(output_ident);
        self.generate(&mut ctx);
        ctx.finish()
    }
}

impl Build for Template {
    fn generate(&self, ctx: &mut Context) {
        self.nodes.iter().for_each(|node| node.generate(ctx));
    }
}

impl Build for Node {
    fn generate(&self, ctx: &mut Context) {
        match self {
            Self::Element(elem) => elem.generate(ctx),
            Self::Block(block) => block.generate(ctx),
            Self::Expr(expr) => expr.generate(ctx),
            Self::If(if_) => if_.generate(ctx),
            Self::Match(match_) => match_.generate(ctx),
            Self::ForLoop(for_loop) => for_loop.generate(ctx),
            Self::Let(let_) => let_.generate(ctx),
        }
    }
}

impl Build for Element {
    fn generate(&self, ctx: &mut Context) {
        if let Err(err) = self.validate() {
            ctx.push(Part::Error(err));
        }

        ctx.push(Part::Raw(format!("<{}", self.name)));

        match self.attributes() {
            Ok(attributes) => {
                for (name, value) in attributes {
                    if let Some(value) = value {
                        ctx.parts.push(Part::Attribute { name, value });
                    } else {
                        ctx.push(Part::Raw(format!(" {name}")));
                    }
                }
            }
            Err(err) => ctx.push(Part::Error(err)),
        }

        ctx.push(Part::Raw(">".into()));

        if let Some(body) = &self.body {
            body.generate(ctx);
            ctx.push(Part::Raw(format!("</{}>", self.name)));
        }
    }
}

impl Build for Block {
    fn generate(&self, ctx: &mut Context) {
        self.nodes.iter().for_each(|node| node.generate(ctx));
    }
}

impl Build for Expr {
    fn generate(&self, ctx: &mut Context) {
        ctx.push(Part::Render(self.clone()));
    }
}

impl Build for If {
    fn generate(&self, ctx: &mut Context) {
        let then_branch = self.then_branch.finish(ctx.output.clone());
        let else_branch = self
            .else_branch
            .as_ref()
            .map(|branch| branch.finish(ctx.output.clone()));

        ctx.push(Part::Stmt(Stmt::Expr(
            ExprIf {
                attrs: Vec::new(),
                if_token: <Token![if]>::default(),
                cond: Box::new(self.cond.clone()),
                then_branch,
                else_branch: else_branch.map(|branch| {
                    (
                        <Token![else]>::default(),
                        Box::new(Expr::Block(ExprBlock {
                            attrs: Vec::new(),
                            label: None,
                            block: branch,
                        })),
                    )
                }),
            }
            .into(),
            Some(<Token![;]>::default()),
        )));
    }
}

impl Build for Match {
    fn generate(&self, ctx: &mut Context) {
        ctx.push(Part::Stmt(Stmt::Expr(
            ExprMatch {
                attrs: Vec::new(),
                match_token: Default::default(),
                expr: Box::new(self.expr.clone()),
                brace_token: Brace::default(),
                arms: self
                    .arms
                    .iter()
                    .map(|arm| syn::Arm {
                        attrs: Vec::new(),
                        pat: arm.pat.clone(),
                        guard: arm
                            .guard
                            .clone()
                            .map(|guard| (Default::default(), Box::new(guard))),
                        body: Box::new(Expr::Block(ExprBlock {
                            attrs: Vec::new(),
                            label: None,
                            block: arm.body.finish(ctx.output.clone()),
                        })),
                        fat_arrow_token: Default::default(),
                        comma: Some(Default::default()),
                    })
                    .collect(),
            }
            .into(),
            Some(<Token![;]>::default()),
        )))
    }
}

impl Build for ForLoop {
    fn generate(&self, ctx: &mut Context) {
        let pat = Box::new(self.pat.clone());
        let expr = Box::new(self.expr.clone());
        let body = self.body.finish(ctx.output.clone());

        ctx.push(Part::Stmt(Stmt::Expr(
            Expr::ForLoop(ExprForLoop {
                attrs: Vec::new(),
                for_token: <Token![for]>::default(),
                label: None,
                in_token: <Token![in]>::default(),
                pat,
                body,
                expr,
            }),
            None,
        )));
    }
}

impl Build for Let {
    fn generate(&self, ctx: &mut Context) {
        ctx.push(Part::Stmt(Stmt::Local(Local {
            attrs: Vec::new(),
            let_token: <Token![let]>::default(),
            pat: self.pat.clone(),
            init: Some(LocalInit {
                eq_token: <Token![=]>::default(),
                expr: Box::new(self.expr.clone()),
                diverge: None,
            }),
            semi_token: <Token![;]>::default(),
        })));
    }
}

struct Context {
    output: Ident,
    parts: Vec<Part>,
}

impl Context {
    fn new(output: Ident) -> Self {
        Self {
            output,
            parts: Vec::new(),
        }
    }

    fn finish(self) -> syn::Block {
        let output = self.output;
        let stmts = self
            .parts
            .into_iter()
            .map(|part| part.into_stmt(&output))
            .collect();

        syn::Block {
            brace_token: Brace::default(),
            stmts,
        }
    }

    fn push(&mut self, part: Part) {
        match (part, self.parts.last_mut()) {
            (Part::Raw(raw), Some(Part::Raw(last))) => last.push_str(&raw),
            (part, _) => self.parts.push(part),
        }
    }
}

enum Part {
    Render(Expr),
    Attribute { name: String, value: Expr },

    Raw(String),
    Stmt(Stmt),

    Error(Error),
}

impl Part {
    fn into_stmt(self, output: &Ident) -> Stmt {
        match self {
            Self::Raw(lit) => {
                parse_quote! { ::std::fmt::Display::fmt(#lit, #output)?; }
            }
            Self::Render(expr) => {
                let expr = ref_with_same_span(expr);

                parse_quote! {
                    ::gen_html::Render::render_to(#expr, #output)?;
                }
            }
            Self::Attribute { name, value } => {
                let value = ref_with_same_span(value);

                parse_quote! {
                    ::gen_html::Value::render_value_to(#value, #name, #output)?;
                }
            }
            Self::Error(err) => parse_quote! { #err },
            Self::Stmt(stmt) => stmt,
        }
    }
}

fn ref_with_same_span(expr: Expr) -> Expr {
    Expr::Reference(syn::ExprReference {
        attrs: Vec::new(),
        and_token: Token![&](expr.span()),
        mutability: None,
        expr: Box::new(expr),
    })
}
