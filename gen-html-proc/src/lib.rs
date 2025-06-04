use syn::parse_macro_input;

mod ast;
mod error;
mod escape;
mod generate;
mod parse;
mod tags;

#[proc_macro]
pub fn html(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let template = parse_macro_input!(input as ast::Template);
    template.expand().into()
}
