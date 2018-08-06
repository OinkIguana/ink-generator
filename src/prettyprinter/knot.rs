use crate::parser::schema::Knot;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{
    multi_zip_expr, nested_tuples_pat, pounded_var_names, quote, quote_each_token, quote_spanned,
};

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
        let segments = print_segments(&stitch.segments, &relative_paths, quote!{ super:: });
        quote! {
            move || {
                #segments
            }
        }
    } else if let Some((name, _)) = knot.stitches.iter().next() {
        let name = Ident::new(&format!("stitch_{}", name), Span::call_site());
        quote! {
            #name(input.clone())
        }
    } else {
        return TokenStream::new();
    };

    quote! {
        mod #name {
            use inkgen::runtime as inkgen;
            pub(super) fn entry(input: inkgen::Rc<inkgen::Cell<usize>>) -> impl inkgen::Generator<Yield = inkgen::Paragraph, Return = ()> {
                #entry
            }
            #(#stitches)*
        }
    }
}
