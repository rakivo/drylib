use proc_macro::{Ident, TokenTree};

#[derive(Debug, PartialEq)]
pub(crate) enum TokenType {
    None,
    Mutable,
    #[allow(dead_code)]
    Immutable,
    MutableReference,
    ImmutableReference
}

pub(crate) type IT = (Ident, TokenType);

pub fn parse_muts<I>(iter: I) -> Vec::<IT>
where I: Iterator<Item = TokenTree>
{
    iter.fold((TokenType::None, Vec::new()), |(ttype, mut ret), t| {
        if let TokenTree::Ident(ident) = t {
            if ident.to_string().eq("mut") {
                if ttype.eq(&TokenType::ImmutableReference) || ttype.eq(&TokenType::MutableReference) {
                    (TokenType::MutableReference, ret)
                } else { (TokenType::Mutable, ret) }
            } else {
                ret.push((ident, ttype));
                (TokenType::None, ret)
            }
        } else if let TokenTree::Punct(punct) = t {
            if punct.as_char().eq(&'&') {
                (TokenType::ImmutableReference, ret)
            } else { (TokenType::None, ret) }
        } else {
            if ttype.eq(&TokenType::Mutable) || ttype.eq(&TokenType::MutableReference) {
                panic!("Expected identifier after `mut`")
            } (TokenType::None, ret)
        }
    }).1
}
