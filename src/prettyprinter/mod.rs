use crate::Ink;
use proc_macro2::{Ident, Span};
use quote::quote;
use uuid::Uuid;

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

    let id = format!("{}", Uuid::new_v4());

    let tokens = quote! {
        pub mod #name {
            #![allow(dead_code, unused_imports, unreachable_code, non_snake_case)]
            use inkgen::{yield_all, runtime};

            pub const ID: runtime::StoryID = runtime::StoryID(#id);

            pub fn story() -> runtime::Story {
                let input = runtime::Input::default();
                let state = runtime::WrappedState::default();
                runtime::Story::new(ID, input.clone(), state.clone(), move || {
                    #entry
                })
            }

            #(#knots)*
        }
    };
    format!("{}", tokens)
}
