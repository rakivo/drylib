use proc_macro::{
    Span,
    Ident,
    Group,
    Punct,
    Spacing,
    Delimiter,
    TokenTree,
    TokenStream
};

use crate::IM;
use crate::clones_prefix::CLONES_PREFIX;

pub fn get_clones_(idents: Vec::<IM>, muts: bool) -> Vec::<TokenTree> {
    let mut ts = Vec::new();
    
    for (ident, mut_) in idents.into_iter() {
        ts.push(TokenTree::Ident(Ident::new("let", Span::call_site())));
        if muts || mut_ { ts.push(TokenTree::Ident(Ident::new("mut", Span::call_site()))); }
        ts.push(TokenTree::Ident(Ident::new(&format!("{CLONES_PREFIX}{ident}"), Span::call_site())));
        ts.push(TokenTree::Punct(Punct::new('=', Spacing::Alone)));
        ts.push(TokenTree::Ident(ident));
        ts.push(TokenTree::Punct(Punct::new('.', Spacing::Alone)));
        ts.push(TokenTree::Ident(Ident::new("clone", Span::call_site())));
        ts.push(TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())));
        ts.push(TokenTree::Punct(Punct::new(';', Spacing::Alone)));
    } ts
}
