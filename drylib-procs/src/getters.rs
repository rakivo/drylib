use proc_macro::{
    Span,
    Ident,
    Punct,
    Spacing,
    Delimiter,
    Group,
    TokenTree,
    TokenStream
};

fn get_getters(fts: Vec<(TokenTree, TokenTree)>, vis: Vec<TokenTree>, muts: bool) -> Vec<TokenTree> {
    let mut braces_group = Vec::new();
    let fn_params = vec![TokenTree::Punct(Punct::new('&', Spacing::Joint)),
                         TokenTree::Ident(Ident::new("self", Span::call_site()))];
            
    let mut_fn_params = vec![TokenTree::Punct(Punct::new('&', Spacing::Joint)),
                             TokenTree::Ident(Ident::new("mut", Span::call_site())),
                             TokenTree::Ident(Ident::new("self", Span::call_site()))];

    let arrow = vec![TokenTree::Punct(Punct::new('-', Spacing::Joint)),
                     TokenTree::Punct(Punct::new('>', Spacing::Alone))];

    for (field, type_) in fts {
        braces_group.extend(vis.clone());
        braces_group.push(TokenTree::Ident(Ident::new("fn", Span::call_site())));
        if muts {
            braces_group.push(TokenTree::Ident(Ident::new(&format!("mut_{field}"), Span::call_site())));
        } else {
            braces_group.push(field.clone());
        }
        if muts {
            braces_group.push(TokenTree::Group(Group::new(Delimiter::Parenthesis,
                                                          TokenStream::from_iter(mut_fn_params.clone()))));
        } else {
            braces_group.push(TokenTree::Group(Group::new(Delimiter::Parenthesis,
                                                          TokenStream::from_iter(fn_params.clone()))));
        }

        braces_group.extend(arrow.clone());
        braces_group.push(TokenTree::Punct(Punct::new('&', Spacing::Joint)));
        if muts { braces_group.push(TokenTree::Ident(Ident::new("mut", Span::call_site()))); }
        braces_group.push(type_.clone());

        let mut fn_body = Vec::new();

        if muts { fn_body.extend(mut_fn_params.clone()); }
        else    { fn_body.extend(fn_params.clone());     }
        fn_body.push(TokenTree::Punct(Punct::new('.', Spacing::Joint)));
        fn_body.push(field);

        braces_group.push(TokenTree::Group(Group::new(Delimiter::Brace,
                                                      TokenStream::from_iter(fn_body.clone()))));
    }

    braces_group
}

pub(crate) fn getters(input: TokenStream, muts: bool) -> Vec::<TokenTree> {
    let tokens = input.into_iter().collect::<Vec<_>>();

    let mut vis = Vec::<TokenTree>::new();
    let mut name = None;
    let mut fts = Vec::<(TokenTree, TokenTree)>::new();
    
    let mut iter = tokens.iter();
    while let Some(token) = iter.next() {
        if let TokenTree::Ident(ident) = token {
            if &ident.to_string() == "pub" {
                vis.push(ident.clone().into());
            } else { name = Some(ident.clone()); }
        } else if let TokenTree::Group(group) = token {
            let mut group_iter = group.stream().into_iter();
            if let Some(TokenTree::Ident(ident)) = group_iter.next() {
                let ident_string = ident.to_string();
                if &ident_string == "self" || &ident_string == "crate" {
                    let tokens: Vec<TokenTree> = vec![ident.into()];
                    let group = Group::new(Delimiter::Parenthesis, TokenStream::from_iter(tokens));
                    vis.push(TokenTree::Group(group));
                } else {
                    group_iter.next(); // `:`
                    fts.push((ident.clone().into(), group_iter.next().expect("Identifier without following type")));
                    while let Some(tt) = group_iter.next() {
                        if let TokenTree::Ident(ident) = tt {
                            group_iter.next(); // `:`
                            if let Some(type_) = group_iter.next() {
                                fts.push((ident.into(), type_));
                            }
                        }
                    }
                }
            }
        }
    }
    
    let braces_group = get_getters(fts, vis, muts);

    vec![TokenTree::from(Ident::new("impl", Span::call_site())),
         name.expect("Failed to find structure's name").into(),
         TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::from_iter(braces_group)))]
}
