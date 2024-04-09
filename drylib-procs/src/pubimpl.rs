use proc_macro::{
    Span,
    Ident,
    Group,
    Delimiter,
    TokenTree,
    TokenStream
};

pub fn pubimpl_parse_impl_<I>(iter: I) -> Vec::<TokenTree>
where I: Iterator<Item = TokenTree>
{
    let (mut ts, mut gs, mut next) = (Vec::new(), Vec::new(), false);
    for tt in iter {
        if next {
            if let TokenTree::Punct(punct) = &tt {
                if punct.as_char() == '>' { next = false; }
            }
            gs.push(tt.clone());
        } else if let TokenTree::Punct(punct) = &tt {
            if punct.as_char().eq(&'<') {
                gs.push(punct.clone().into());
                next = true;
            }
        } else if let TokenTree::Group(group) = &tt {
            let mut temp = Vec::new();
            for tt in group.stream().into_iter() {
                if let TokenTree::Ident(ident) = &tt {
                    if ident.to_string() == "fn" {
                        temp.push((Ident::new("pub", Span::call_site())).into());
                    }
                } temp.push(tt);
            }
            ts.push(TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::from_iter(temp))));
            continue;
        } ts.push(tt);
    }

    let mut ret = vec![TokenTree::from(Ident::new("impl", Span::call_site()))];
    ret.extend(gs);
    ret.extend(ts);
    ret
}
