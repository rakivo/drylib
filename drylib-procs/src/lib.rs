extern crate proc_macro;

use proc_macro::TokenStream;

#[cfg(feature = "muts")]
mod muts;
#[cfg(feature = "clones")]
mod clones;
mod clones_prefix;
#[cfg(feature = "pubimpl")]
mod pubimpl;

#[proc_macro]
#[cfg(feature = "muts")]
pub fn muts(input: TokenStream) -> TokenStream {
    use crate::muts::*;
    let (ids, ids_len, vals, vals_len) = muts_get_ids_vals_(&mut input.into_iter());
    assert_eq!(ids_len, vals_len, "The number of idents and values must be the same");
    TokenStream::from_iter(get_muts_(ids, vals))
}

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
#[cfg(feature = "pubimpl")]
pub fn pubimpl(input: TokenStream) -> TokenStream {
    use crate::pubimpl::*;
    let impl_ = pubimpl_parse_impl_(input.into_iter());
    TokenStream::from_iter(impl_)
}
