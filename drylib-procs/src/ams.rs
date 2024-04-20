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
use crate::ams_prefix::AMS_PREFIX;

pub fn get_ams_(idents: Vec::<IT>, muts: bool) -> Vec::<TokenTree> {
    let mut ts = Vec::new();
    
    for (ident, ttype) in idents.into_iter() {
        ts.push(TokenTree::Ident(Ident::new("let", Span::call_site())));
        if muts || ttype.eq(&TokenType::Mutable) || ttype.eq(&TokenType::MutableReference) {
            ts.push(TokenTree::Ident(Ident::new("mut", Span::call_site())));
        }
        ts.push(TokenTree::Ident(Ident::new(&format!("{AMS_PREFIX}{ident}"), Span::call_site())));
        ts.push(TokenTree::Punct(Punct::new('=', Spacing::Alone)));

        ts.push(TokenTree::Ident(Ident::new("std", Span::call_site())));
        ts.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
        ts.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
        ts.push(TokenTree::Ident(Ident::new("sync", Span::call_site())));
        ts.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
        ts.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
        ts.push(TokenTree::Ident(Ident::new("Arc", Span::call_site())));
        ts.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
        ts.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
        ts.push(TokenTree::Ident(Ident::new("new", Span::call_site())));

        let mut group = Vec::new();
        group.push(TokenTree::Ident(Ident::new("std", Span::call_site())));
        group.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
        group.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
        group.push(TokenTree::Ident(Ident::new("sync", Span::call_site())));
        group.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
        group.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
        group.push(TokenTree::Ident(Ident::new("Mutex", Span::call_site())));
        group.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
        group.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
        group.push(TokenTree::Ident(Ident::new("new", Span::call_site())));

        let ident = if ttype.eq(&TokenType::ImmutableReference) || ttype.eq(&TokenType::MutableReference) {
            vec![TokenTree::Punct(Punct::new('&', Spacing::Alone)), TokenTree::Ident(ident)]
        } else {
            vec![TokenTree::Ident(ident)]
        };
        group.push(TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::from_iter(ident))));

        ts.push(TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::from_iter(group))));
        ts.push(TokenTree::Punct(Punct::new(';', Spacing::Alone)));
    } ts
}

