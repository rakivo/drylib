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

use crate::parse::IT;
use crate::parse::TokenType;
use crate::clones_prefix::CLONES_PREFIX;

pub fn get_clones_(idents: Vec::<IT>, muts: bool) -> Vec::<TokenTree> {
    let mut ts = Vec::new();
    
    for (ident, ttype) in idents.into_iter() {
        ts.push(TokenTree::Ident(Ident::new("let", Span::call_site())));
        if muts || ttype.eq(&TokenType::Mutable) {
            ts.push(TokenTree::Ident(Ident::new("mut", Span::call_site())));
        }
        ts.push(TokenTree::Ident(Ident::new(&format!("{CLONES_PREFIX}{ident}"), Span::call_site())));
        ts.push(TokenTree::Punct(Punct::new('=', Spacing::Alone)));
        if ttype.eq(&TokenType::MutableReference) {
            ts.push(TokenTree::Ident(Ident::new("mut", Span::call_site())));
        } else if ttype.eq(&TokenType::ImmutableReference) || ttype.eq(&TokenType::MutableReference) {
            ts.push(TokenTree::Punct(Punct::new('&', Spacing::Alone)));
        }
        ts.push(TokenTree::Ident(ident));
        ts.push(TokenTree::Punct(Punct::new('.', Spacing::Alone)));
        ts.push(TokenTree::Ident(Ident::new("clone", Span::call_site())));
        ts.push(TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())));
        ts.push(TokenTree::Punct(Punct::new(';', Spacing::Alone)));
    } ts
}
