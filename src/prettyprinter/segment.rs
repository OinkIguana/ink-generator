use crate::parser::schema::{Message, Part, Segment};
use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use quote::{
    multi_zip_expr, nested_tuples_pat, pounded_var_names, quote, quote_each_token, quote_spanned,
};

fn find_divert(parts: &Vec<Part>) -> Option<(usize, &String)> {
    parts.iter().enumerate().find_map(|(i, part)| {
        if let Part::Divert(Some(target)) = part {
            Some((i, target))
        } else {
            None
        }
    })
}

fn break_at_break(parts: &[Part]) -> Vec<&[Part]> {
    let mut breaks = vec![];
    let mut last = 0;
    for (i, part) in parts.iter().enumerate() {
        if part == &Part::Break {
            if last != i {
                breaks.push(&parts[last..i]);
            }
            last = i + 1;
        }
    }
    if last != parts.len() {
        breaks.push(&parts[last..]);
    }
    breaks
}

fn divert_to(
    root_path: TokenStream,
    knot: Option<&str>,
    stitch: Option<&str>,
    parts: Vec<Part>,
) -> TokenStream {
    let knot_name = knot.map(|knot| Ident::new(&format!("knot_{}", knot), Span::call_site()));
    let stitch_name =
        stitch.map(|stitch| Ident::new(&format!("stitch_{}", stitch), Span::call_site()));
    let path = match (knot_name, stitch_name) {
        (Some(knot), None) => quote! { #root_path #knot::entry },
        (None, Some(stitch)) => quote! { #stitch },
        (Some(knot), Some(stitch)) => quote! { #root_path #knot::#stitch },
        (None, None) => panic!("Must supply at least one of knot or stitch to perform a divert"),
    };

    let mut output = TokenStream::new();
    let mut last = vec![];
    if !parts.is_empty() {
        let breaks = break_at_break(&parts);
        for parts in &breaks[..breaks.len() - 1] {
            output = quote! {
                #output
                yield inkgen::Paragraph::new(vec![#(#parts),*], None);
            };
        }
        last = breaks[breaks.len() - 1].to_vec();
    }

    quote! {
        #output
        let continuation = inkgen::Paragraph::new(vec![#(#last),*], None);
        let mut gen = #path(input.clone());
        match unsafe { inkgen::Generator::resume(&mut gen) } {
            inkgen::GeneratorState::Yielded(paragraph) => {
                yield continuation.join(paragraph);
                yield_all! { gen }
            }
            inkgen::GeneratorState::Complete(()) => yield continuation,
        }
    }
}

crate fn print_segments(
    segments: &Vec<Segment>,
    relative_paths: &Vec<&String>,
    root_path: TokenStream,
) -> TokenStream {
    let mut output = TokenStream::new();
    let mut parts = vec![];
    for segment in segments {
        match segment {
            Segment::Text(message) => {
                parts = [parts, message.parts.clone()].concat();
                // TODO: tunnel support here
                if let Some((i, divert)) = find_divert(&parts) {
                    if let Some(split) = divert.find(".") {
                        // full divert path
                        let (knot, stitch) = divert.split_at(split);
                        let diverted = divert_to(
                            root_path,
                            Some(knot),
                            Some(&stitch[1..]),
                            parts[..i].to_vec(),
                        );
                        return quote! { #output #diverted };
                    } else if relative_paths.contains(&divert) {
                        // relative path
                        let diverted =
                            divert_to(root_path, None, Some(divert), parts[..i].to_vec());
                        return quote! { #output #diverted };
                    } else if divert == "DONE" || divert == "END" {
                        // not a real divert
                        let breaks = break_at_break(&parts[..i]);
                        for parts in &breaks {
                            output = quote! {
                                #output
                                yield inkgen::Paragraph::new(vec![#(#parts),*], None);
                            };
                        }
                        return quote! { #output return };
                    } else {
                        // full divert path to just a knot
                        let diverted =
                            divert_to(root_path, Some(divert), None, parts[..i].to_vec());
                        return quote! { #output #diverted };
                    }
                }
            }
            Segment::Choices(choices) => {
                let options: Vec<_> = choices
                    .iter()
                    .map(|(choice, _)| {
                        let parts =
                            [choice.prefix.parts.clone(), choice.choice.parts.clone()].concat();
                        quote! { vec![#(#parts),*] }
                    }).collect();
                let cases: Vec<_> = choices
                    .iter()
                    .enumerate()
                    .map(|(index, (choice, case))| {
                        let segment = Segment::Text(Message {
                            parts: [
                                choice.prefix.parts.clone(),
                                choice.suffix.parts.clone(),
                                vec![Part::Break],
                            ]
                                .concat(),
                        });
                        let tokens = print_segments(
                            &[vec![segment], case.clone()].concat(),
                            relative_paths,
                            root_path.clone(),
                        );
                        quote! {
                            #index => {
                                #tokens
                                break
                            }
                        }
                    }).collect();
                let breaks = break_at_break(&parts);
                for parts in &breaks[0..breaks.len() - 1] {
                    output = quote! {
                        #output
                        yield inkgen::Paragraph::new(vec![#(#parts),*], None);
                    };
                }
                let last = breaks[breaks.len() - 1];
                output = quote! {
                    #output
                    loop {
                        yield inkgen::Paragraph::new(vec![#(#last),*], Some(vec![#(#options),*]));
                        let choice = input.get();
                        match choice {
                            #(#cases)*
                            _ => continue,
                        }
                    }
                };
                parts.clear();
            }
        }
    }
    let breaks = break_at_break(&parts);
    for parts in &breaks {
        output = quote! {
            #output
            yield inkgen::Paragraph::new(vec![#(#parts),*], None);
        };
    }
    output
}

impl ToTokens for Part {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Part::Divert(..) => panic!("Cannot turn a Divert to Tokens"),
            Part::Text(string) => {
                let copy = tokens.clone();
                *tokens = quote! { #copy inkgen::Part::Text(String::from(#string)) };
            }
            Part::Glue => {
                let copy = tokens.clone();
                *tokens = quote! { #copy inkgen::Part::Glue };
            }
            Part::Break => {
                let copy = tokens.clone();
                *tokens = quote! { #copy inkgen::Part::Break };
            }
        }
    }
}
