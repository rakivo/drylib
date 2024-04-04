 use proc_macro::{
    Span,
    Ident,
    Punct,
    Spacing,
    Literal,
    TokenTree
};

pub fn get_muts_(ids: Vec::<Ident>, vals: Vec::<Literal>) -> Vec::<TokenTree> {
    let mut ts = Vec::new();

    for (ident, value) in ids.into_iter().zip(vals.into_iter()) {
        ts.push(TokenTree::Ident(Ident::new("let", Span::call_site())));
        ts.push(TokenTree::Ident(Ident::new("mut", Span::call_site())));
        ts.push(TokenTree::Ident(ident));
        ts.push(TokenTree::Punct(Punct::new('=', Spacing::Alone)));
        ts.push(TokenTree::Literal(value));
        ts.push(TokenTree::Punct(Punct::new(';', Spacing::Alone)));
    } ts
}

pub fn muts_get_ids_vals_<I>(iter: &mut I) -> (Vec::<Ident>, usize, Vec::<Literal>, usize)
where I: Iterator<Item = TokenTree>
{
    let (mut ids, mut ids_len, mut vals, mut vals_len) = (Vec::new(), 0, Vec::new(), 0);
    
    while let Some(tt) = iter.next() {
        if let TokenTree::Ident(ident) = tt {
            println!("ident: {ident}");
            ids.push(ident);
            ids_len += 1;
            if let Some(TokenTree::Punct(punct)) = iter.next() {
                println!("punct: {punct}");
                if punct.as_char().eq(&'=') {
                    if let Some(TokenTree::Literal(liter)) = iter.next() {
                        println!("literal: {liter}");
                        vals.push(liter);
                        vals_len += 1;
                        if let Some(TokenTree::Punct(next_punct)) = iter.next() {
                            assert_eq!(next_punct.as_char(), ',', "Expected ',' after value");
                        }
                    }
                } else { panic!("Expected '=' after identifier") }
            } else { panic!("Expected '=' after identifier") }
        } else {
            println!("tt: {tt}");
            panic!("Expected identifier")
        }
    } (ids, ids_len, vals, vals_len)
}
