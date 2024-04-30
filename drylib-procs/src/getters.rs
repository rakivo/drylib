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

fn get_getters(fields: Vec::<TokenTree>, types: Vec::<Vec::<TokenTree>>, vis: Vec::<TokenTree>, muts: bool) -> Vec::<TokenTree> {
    let mut braces_group = Vec::new();
    let fn_params = vec![TokenTree::Punct(Punct::new('&', Spacing::Joint)),
                         TokenTree::Ident(Ident::new("self", Span::call_site()))];
            
    let mut_fn_params = vec![TokenTree::Punct(Punct::new('&', Spacing::Joint)),
                             TokenTree::Ident(Ident::new("mut", Span::call_site())),
                             TokenTree::Ident(Ident::new("self", Span::call_site()))];

    let arrow = vec![TokenTree::Punct(Punct::new('-', Spacing::Joint)),
                     TokenTree::Punct(Punct::new('>', Spacing::Alone))];

    for (field, types) in fields.iter().zip(types.iter()) {
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
        braces_group.extend(types.clone());

        let mut fn_body = Vec::new();
        if muts { fn_body.extend(mut_fn_params.clone()); }
        else    { fn_body.extend(fn_params.clone());     }
        fn_body.push(TokenTree::Punct(Punct::new('.', Spacing::Joint)));
        fn_body.push(field.clone());

        braces_group.push(TokenTree::Group(Group::new(Delimiter::Brace,
                                                      TokenStream::from_iter(fn_body.clone()))));
    }

    braces_group
}

pub(crate) fn getters(input: TokenStream, muts: bool) -> Vec::<TokenTree> {
    let tokens = input.into_iter().collect::<Vec<_>>();

    let mut vis = Vec::<TokenTree>::new();
    let mut name = None;
    let mut vis_flag = false;
    let mut fields = Vec::<TokenTree>::new();
    let mut types = Vec::<Vec<TokenTree>>::new();
    
    let mut iter = tokens.iter();
    while let Some(token) = iter.next() {
        if let TokenTree::Ident(ident) = token {
            if &ident.to_string() == "pub" {
                vis.push(ident.clone().into());
            } else { name = Some(ident.clone()); }
        } else if let TokenTree::Group(group) = token {
            let mut group_iter = group.stream().into_iter();
            if !vis_flag {
                let ident = group_iter.next().unwrap();
                let ident_string = ident.to_string();
                if &ident_string == "self" || &ident_string == "crate" {
                    let tokens: Vec::<TokenTree> = vec![ident.clone().into()];
                    let group = Group::new(Delimiter::Parenthesis, TokenStream::from_iter(tokens));
                    vis.push(TokenTree::Group(group));
                } vis_flag = true;
            } else {
                let mut next_field = true;
                let mut next_type = false;
                let mut temp = Vec::new();
                while let Some(tt) = group_iter.next() {
                    if next_type {
                        temp.push(tt.clone());
                    } else if next_field {
                        fields.push(tt.clone());
                        next_field = false;
                    } if let TokenTree::Punct(p) = tt {
                        match p.as_char() {
                            ':' => next_type = true,
                            ';' => {
                                next_field = true;
                                next_type = false;
                                let temp_ = temp.iter().take(temp.len() - 1).cloned().collect::<Vec<_>>();
                                types.push(temp_);
                                temp.clear()
                            }
                            _   => {}
                        }
                    }
                }
            }
        }
    }
    
    let braces_group = get_getters(fields, types, vis, muts);

    let ts = vec![TokenTree::from(Ident::new("impl", Span::call_site())),
         name.expect("Failed to find structure's name").into(),
                  TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::from_iter(braces_group)))];
    ts
}
