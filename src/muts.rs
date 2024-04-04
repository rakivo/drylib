 use proc_macro::{
    Span,
    Ident,
    Punct,
    Spacing,
    TokenTree
};

pub fn get_muts_(ids: Vec::<Ident>, vals: Vec::<Vec::<TokenTree>>) -> Vec::<TokenTree> {
    let mut ts = Vec::new();

    for (ident, value) in ids.into_iter().zip(vals.into_iter()) {
        ts.push(TokenTree::Ident(Ident::new("let", Span::call_site())));
        ts.push(TokenTree::Ident(Ident::new("mut", Span::call_site())));
        ts.push(TokenTree::Ident(ident));
        ts.push(TokenTree::Punct(Punct::new('=', Spacing::Alone)));
        value.into_iter().for_each(|tt| ts.push(tt));
        ts.push(TokenTree::Punct(Punct::new(';', Spacing::Alone)));
    } ts
}

pub fn muts_get_ids_vals_<I>(iter: &mut I) -> (Vec::<Ident>, usize, Vec::<Vec::<TokenTree>>, usize)
where I: Iterator<Item = TokenTree>
{
    let (mut ids, mut ids_len, mut vals, mut vals_len) = (Vec::new(), 0, Vec::new(), 0);
    
    while let Some(tt) = iter.next() {
        if let TokenTree::Ident(ident) = tt {
            ids.push(ident);
            ids_len += 1;
            if let Some(TokenTree::Punct(punct)) = iter.next() {
                if punct.as_char().eq(&'=') {
                    let mut temp = Vec::new();
                    loop {
                        if let Some(next) = iter.next() {
                            if let TokenTree::Punct(punct) = &next {
                                if punct.as_char() == ';' { break; }
                            }
                            temp.push(next);
                        } else { break; }
                    }
                    vals.push(temp);
                    vals_len += 1;
                } else { panic!("Expected '=' after identifier") }
            } else { panic!("Expected '=' after identifier") }
        } else { panic!("Expected identifier") }
    } (ids, ids_len, vals, vals_len)
}
