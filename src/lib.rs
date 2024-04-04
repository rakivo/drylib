extern crate proc_macro;

use proc_macro::TokenStream;

mod clones_prefix;
#[cfg(feature = "clones")]
mod clones;
#[cfg(feature = "muts")]
mod muts;

#[proc_macro]
#[cfg(feature = "clones")]
pub fn clones(input: TokenStream) -> TokenStream {
    use crate::clones::*;
    let ids = clones_parse_muts_(input.into_iter());
    TokenStream::from_iter(get_clones_(ids, false))
}

#[proc_macro]
#[cfg(feature = "mutclones")]
pub fn mutclones(input: TokenStream) -> TokenStream {
    use crate::clones::*;
    let ids = clones_parse_muts_(input.into_iter());
    TokenStream::from_iter(get_clones_(ids, true))
}

#[proc_macro]
#[cfg(feature = "muts")]
pub fn muts(input: TokenStream) -> TokenStream {
    use crate::muts::*;
    #[allow(unused)]
    let mut iter = input.into_iter();
    let (ids, ids_len, vals, vals_len) = muts_get_ids_vals_(&mut iter);
    assert_eq!(ids_len, vals_len, "The number of idents and values must be the same");
    TokenStream::from_iter(get_muts_(ids, vals))
}
