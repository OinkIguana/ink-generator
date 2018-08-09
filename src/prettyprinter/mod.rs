use crate::Ink;
use proc_macro2::{Ident, Span};
use quote::{
    multi_zip_expr, nested_tuples_pat, pounded_var_names, quote, quote_each_token, quote_spanned,
};

mod knot;
mod segment;
mod stitch;

use self::knot::print_knot;
use self::segment::print_segments;

pub fn pretty_print(name: &str, ink: Ink) -> String {
    let name = Ident::new(name, Span::call_site());
    let entry = print_segments(
        &ink.entry,
        &ink.knots.iter().map(|(name, _)| name).collect::<Vec<_>>(),
        false,
    );
    let knots = ink.knots.iter().map(|(name, knot)| print_knot(name, knot));

    let tokens = quote! {
        pub mod #name {
            #![allow(dead_code, unused_imports, unreachable_code, non_snake_case)]
            use inkgen::yield_all;
            use inkgen::runtime as inkgen;

            pub fn story() -> inkgen::Story {
                let input = inkgen::Input::default();
                let state = inkgen::WrappedState::default();
                inkgen::Story::new(input.clone(), state.clone(), move || {
                    #entry
                })
            }

            #(#knots)*
        }
    };
    format!("{}", tokens)
}
