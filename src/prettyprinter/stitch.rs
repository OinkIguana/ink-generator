use super::segment::print_segments;
use crate::parser::schema::Stitch;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

crate fn print_stitch(name: &str, stitch: &Stitch, relative_paths: &Vec<&String>) -> TokenStream {
    let name = Ident::new(&format!("stitch_{}", name), Span::call_site());
    let segments = print_segments(&stitch.segments, relative_paths, true);
    quote! {
        pub(super) fn #name(input: inkgen::runtime::Input, state: inkgen::runtime::WrappedState) -> impl inkgen::runtime::Generator<Yield = inkgen::runtime::Paragraph, Return = ()> + Sync + Send {
            move || {
                #(#segments)*
            }
        }
    }
}
