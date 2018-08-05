use super::segment::print_segments;
use crate::parser::schema::Stitch;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{
    multi_zip_expr, nested_tuples_pat, pounded_var_names, quote, quote_each_token, quote_spanned,
};

crate fn print_stitch(name: &str, stitch: &Stitch) -> TokenStream {
    let name = Ident::new(&format!("stitch_{}", name), Span::call_site());
    let segments = print_segments(&stitch.segments);
    quote! {
        fn #name(input: inkgen::Rc<inkgen::Cell<usize>>) -> impl inkgen::Generator<Yield = inkgen::Paragraph, Return = ()> {
            move || {
                #(#segments)*
            }
        }
    }
}
