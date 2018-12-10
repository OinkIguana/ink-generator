use crate::parser::schema::Knot;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use super::segment::print_segments;
use super::stitch::print_stitch;

crate fn print_knot(name: &str, knot: &Knot) -> TokenStream {
    let name = Ident::new(&format!("knot_{}", name), Span::call_site());

    let relative_paths = knot
        .stitches
        .iter()
        .map(|(name, _)| name)
        .collect::<Vec<_>>();

    let stitches = knot
        .stitches
        .iter()
        .map(|(name, stitch)| print_stitch(name, stitch, &relative_paths));

    let entry = if let Some(stitch) = &knot.entry {
        let segments = print_segments(&stitch.segments, &relative_paths, true);
        quote! {
            move || {
                #segments
            }
        }
    } else if let Some((name, _)) = knot.stitches.iter().next() {
        let name = Ident::new(&format!("stitch_{}", name), Span::call_site());
        quote! {
            #name(input, state)
        }
    } else {
        return TokenStream::new();
    };

    quote! {
        mod #name {
            use inkgen::yield_all;
            pub(super) fn entry(input: inkgen::runtime::Input, state: inkgen::runtime::WrappedState) -> impl inkgen::runtime::Generator<Yield = inkgen::runtime::Paragraph, Return = ()> + Sync + Send {
                #entry
            }
            #(#stitches)*
        }
    }
}
