extern crate proc_macro;

use proc_macro::TokenStream;

mod parse;
use parse::parse_muts;

#[cfg(feature = "ams")]
mod ams;
mod ams_prefix;
#[cfg(feature = "muts")]
mod muts;
#[cfg(feature = "clones")]
mod clones;
mod clones_prefix;
#[cfg(feature = "pubimpl")]
mod pubimpl;

#[proc_macro]
#[cfg(feature = "ams")]
pub fn ams(input: TokenStream) -> TokenStream {
    use crate::ams::*;
    let ids = parse_muts(input.into_iter());
    TokenStream::from_iter(get_ams_(ids, false))
}

#[proc_macro]
#[cfg(feature = "mutams")]
pub fn mutams(input: TokenStream) -> TokenStream {
    use crate::ams::*;
    let ids = parse_muts(input.into_iter());
    TokenStream::from_iter(get_ams_(ids, true))
}

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
    let ids = parse_muts(input.into_iter());
    TokenStream::from_iter(get_clones_(ids, false))
}

#[proc_macro]
#[cfg(feature = "mutclones")]
pub fn mutclones(input: TokenStream) -> TokenStream {
    use crate::clones::*;
    let ids = parse_muts(input.into_iter());
    TokenStream::from_iter(get_clones_(ids, true))
}

#[proc_macro]
#[cfg(feature = "pubimpl")]
pub fn pubimpl(input: TokenStream) -> TokenStream {
    use crate::pubimpl::*;
    let impl_ = pubimpl_parse_impl_(input.into_iter());
    TokenStream::from_iter(impl_)
}
