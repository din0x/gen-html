use crate::{
    ast::{Block, Element, ForLoop, If, Match, Node, Template},
    error::Error,
    escape::escape_html,
};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    Expr, ExprBlock, ExprForLoop, ExprIf, ExprLit, ExprMatch, Ident, Lit, Stmt, Token, parse_quote,
    token::Brace,
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

trait Generate {
    fn generate(&self, g: &mut Generator);

    fn finish(&self, output_ident: Ident) -> syn::Block {
        let mut g = Generator::new(output_ident);
        self.generate(&mut g);
        g.finish()
    }
}

impl Generate for Template {
    fn generate(&self, g: &mut Generator) {
        self.nodes.iter().for_each(|node| node.generate(g));
    }
}

impl Generate for Node {
    fn generate(&self, g: &mut Generator) {
        match self {
            Self::Element(elem) => elem.generate(g),
            Self::Block(block) => block.generate(g),
            Self::Expr(expr) => expr.generate(g),
            Self::If(if_) => if_.generate(g),
            Self::Match(match_) => match_.generate(g),
            Self::ForLoop(for_loop) => for_loop.generate(g),
        }
    }
}

impl Generate for Element {
    fn generate(&self, g: &mut Generator) {
        if let Err(err) = self.validate() {
            g.push_error(err);
        }

        g.push_str("<");
        g.push_str(&self.name.to_string());

        match self.attributes() {
            Ok(attributes) => {
                for attr in attributes {
                    g.push_str(" ");
                    g.push_str(&attr.0);

                    if let Some(value) = attr.1 {
                        g.push_str("=\"");
                        g.push_rendered_expr(value.clone());
                        g.push_str("\"");
                    }
                }
            }
            Err(err) => g.push_error(err),
        }

        g.push_str(">");

        if let Some(body) = &self.body {
            body.generate(g);
            g.push_str("</");
            g.push_str(&self.name.to_string());
            g.push_str(">");
        }
    }
}

impl Generate for Block {
    fn generate(&self, g: &mut Generator) {
        self.nodes.iter().for_each(|node| node.generate(g));
    }
}

impl Generate for Expr {
    fn generate(&self, g: &mut Generator) {
        g.push_rendered_expr(self.clone());
    }
}

impl Generate for If {
    fn generate(&self, g: &mut Generator) {
        let then_branch = self.then_branch.finish(g.output_ident.clone());
        let else_branch = self
            .else_branch
            .as_ref()
            .map(|branch| branch.finish(g.output_ident.clone()));

        g.push_stmt(Stmt::Expr(
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
        ));
    }
}

impl Generate for Match {
    fn generate(&self, g: &mut Generator) {
        g.push_stmt(Stmt::Expr(
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
                            block: arm.body.finish(g.output_ident.clone()),
                        })),
                        fat_arrow_token: Default::default(),
                        comma: Some(Default::default()),
                    })
                    .collect(),
            }
            .into(),
            Some(<Token![;]>::default()),
        ))
    }
}

impl Generate for ForLoop {
    fn generate(&self, g: &mut Generator) {
        let pat = Box::new(self.pat.clone());
        let expr = Box::new(self.expr.clone());
        let body = self.body.finish(g.output_ident.clone());

        g.push_stmt(Stmt::Expr(
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
        ));
    }
}

struct Generator {
    output_ident: Ident,
    parts: Vec<Part>,
}

impl Generator {
    fn new(output_ident: Ident) -> Self {
        Self {
            output_ident,
            parts: Vec::new(),
        }
    }

    fn finish(self) -> syn::Block {
        let output_ident = self.output_ident;
        let mut stmts = Vec::new();

        for part in self.parts {
            match part {
                Part::Static(lit) => {
                    stmts.push(parse_quote! {
                        ::std::fmt::Display::fmt(#lit, #output_ident)?;
                    });
                }
                Part::Dynamic(dynamic_stmt) => stmts.push(dynamic_stmt),
            }
        }

        syn::Block {
            brace_token: Brace::default(),
            stmts,
        }
    }

    fn push_str(&mut self, s: &str) {
        if let Some(Part::Static(lit)) = self.parts.last_mut() {
            lit.push_str(s);
        } else {
            self.parts.push(Part::Static(s.to_owned()));
        }
    }

    fn push_rendered_expr(&mut self, expr: Expr) {
        let output_ident = &self.output_ident;

        match expr {
            Expr::Lit(ExprLit {
                lit: Lit::Str(lit), ..
            }) => self.push_str(&escape_html(&lit.value())),
            expr => self.push_stmt(parse_quote! {
                ::gen_html::Render::render_to(&#expr, #output_ident)?;
            }),
        }
    }

    fn push_stmt(&mut self, stmt: Stmt) {
        self.parts.push(Part::Dynamic(stmt));
    }

    fn push_error(&mut self, err: Error) {
        self.parts.push(Part::Dynamic(parse_quote! { #err }));
    }
}

enum Part {
    Static(String),
    Dynamic(Stmt),
}
