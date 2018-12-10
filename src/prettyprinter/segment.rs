use crate::parser::schema::{Message, Part, Segment, StoryPoint};
use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use quote::quote;

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
    is_in_module: bool,
    knot: Option<&str>,
    stitch: Option<&str>,
    parts: Vec<Part>,
) -> TokenStream {
    let (knot_name, stitch_name) = {
        if is_in_module {
            (
                knot.map(|knot| Ident::new(&format!("knot_{}", knot), Span::call_site())),
                stitch.map(|stitch| Ident::new(&format!("stitch_{}", stitch), Span::call_site())),
            )
        } else {
            (
                stitch.map(|knot| Ident::new(&format!("knot_{}", knot), Span::call_site())),
                None,
            )
        }
    };
    let root_path = if is_in_module { quote! { super:: } } else { quote!{} };

    let path = match (knot_name, stitch_name) {
        (Some(knot), None) => quote! { #root_path #knot::entry },
        (None, Some(stitch)) => quote! { #stitch },
        (Some(knot), Some(stitch)) => quote! { #root_path #knot::#stitch },
        (None, None) => panic!("Must supply at least one of knot or stitch to perform a divert"),
    };

    let mut output = TokenStream::new();
    let mut last = vec![];
    if !parts.is_empty() {
        let mut breaks = break_at_break(&parts);
        if parts.last() != Some(&Part::Break) {
            last = breaks.pop().unwrap().to_vec();
            last.push(Part::Glue);
        }
        for parts in &breaks {
            output = quote! {
                #output
                yield inkgen::runtime::Paragraph::new(vec![#(#parts),*], None);
            };
        }
    }

    quote! {
        #output
        let continuation = inkgen::runtime::Paragraph::new(vec![#(#last),*], None);
        let mut gen: Box<dyn inkgen::runtime::Generator<Yield = inkgen::runtime::Paragraph, Return = ()> + Sync + Send> = Box::new(#path(input, state));
        match unsafe { inkgen::runtime::Generator::resume(&mut gen) } {
            inkgen::runtime::GeneratorState::Yielded(paragraph) => {
                yield continuation.join(paragraph);
                yield_all! { gen }
            }
            inkgen::runtime::GeneratorState::Complete(()) => yield continuation,
        }
    }
}

crate fn print_segments(
    segments: &Vec<Segment>,
    relative_paths: &Vec<&String>,
    is_in_module: bool,
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
                            is_in_module,
                            Some(knot),
                            Some(&stitch[1..]),
                            parts[..i].to_vec(),
                        );
                        return quote! { #output #diverted };
                    } else if relative_paths.contains(&divert) {
                        // relative path
                        let diverted = divert_to(is_in_module, None, Some(divert), parts[..i].to_vec());
                        return quote! { #output #diverted };
                    } else if divert == "DONE" || divert == "END" {
                        // not a real divert
                        let breaks = break_at_break(&parts[..i]);
                        for parts in &breaks {
                            output = quote! {
                                #output
                                yield inkgen::runtime::Paragraph::new(vec![#(#parts),*], None);
                            };
                        }
                        return quote! { #output return; };
                    } else {
                        // full divert path to just a knot
                        let diverted = divert_to(is_in_module, Some(divert), None, parts[..i].to_vec());
                        return quote! { #output #diverted };
                    }
                }
            }
            Segment::Choices(choices) => {
                let stickies: Vec<_> = choices
                    .iter()
                    .map(|(choice, _)| choice.sticky)
                    .collect();
                let names: Vec<_> = choices
                    .iter()
                    .map(|(choice, _)| &choice.name)
                    .collect();
                let options: Vec<_> = choices
                    .iter()
                    .map(|(choice, _)| {
                        let parts = [choice.prefix.parts.clone(), choice.choice.parts.clone()].concat();
                        quote! { vec![#(#parts),*] }
                    })
                    .collect();
                let cases: Vec<_> = choices
                    .iter()
                    .map(|(choice, case)| {
                        let segment = Segment::Text(Message {
                            parts: [
                                choice.prefix.parts.clone(),
                                choice.suffix.parts.clone(),
                                vec![Part::Break],
                            ].concat(),
                        });
                        let tokens = print_segments(
                            &[vec![segment], case.clone()].concat(),
                            relative_paths,
                            is_in_module,
                        );
                        let name = &choice.name;
                        let sticky = choice.sticky;
                        quote! {
                            if #sticky || !state.lock().unwrap().visited(#name) {
                                i += 1;
                            }
                            if i == choice {
                                state.lock().unwrap().visit(#name);
                                #tokens
                                break;
                            }
                        }
                    })
                    .collect();
                let breaks = break_at_break(&parts);
                for parts in &breaks[0..breaks.len() - 1] {
                    output = quote! {
                        #output
                        yield inkgen::runtime::Paragraph::new(vec![#(#parts),*], None);
                    };
                }
                let last = breaks[breaks.len() - 1];
                output = quote! {
                    #output
                    loop {
                        let choices = {
                            let state = state.lock().unwrap();
                            let mut choices = vec![];
                            #(
                                if #stickies || !state.visited(#names) {
                                    choices.push(#options);
                                }
                            )*
                            choices
                        };
                        yield inkgen::runtime::Paragraph::new(
                            vec![#(#last),*],
                            Some(choices),
                        );
                        let choice = *input.lock().unwrap();
                        let mut i = 0;
                        #(#cases)*
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
            yield inkgen::runtime::Paragraph::new(vec![#(#parts),*], None);
        };
    }
    output
}

impl ToTokens for StoryPoint {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let copy = tokens.clone();
        match self {
            StoryPoint::Named(string) => {
                *tokens = quote! { #copy inkgen::runtime::StoryPoint::Named(#string) };
            }
            StoryPoint::Unnamed(string) => {
                *tokens = quote! { #copy inkgen::runtime::StoryPoint::Unnamed(#string) };
            }
        }
    }
}

impl ToTokens for Part {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Part::Divert(..) => panic!("Cannot turn a Divert to Tokens"),
            Part::Text(string) => {
                let copy = tokens.clone();
                *tokens = quote! { #copy inkgen::runtime::Part::Text(#string) };
            }
            Part::Tag(string) => {
                let copy = tokens.clone();
                *tokens = quote! { #copy inkgen::runtime::Part::Tag(#string) };
            }
            Part::Glue => {
                let copy = tokens.clone();
                *tokens = quote! { #copy inkgen::runtime::Part::Glue };
            }
            Part::Break => {
                let copy = tokens.clone();
                *tokens = quote! { #copy inkgen::runtime::Part::Break };
            }
        }
    }
}
