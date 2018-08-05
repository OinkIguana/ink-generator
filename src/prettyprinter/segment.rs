use crate::parser::schema::{Message, Part, Segment};
use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::{
    multi_zip_expr, nested_tuples_pat, pounded_var_names, quote, quote_each_token, quote_spanned,
};

fn find_divert(parts: &Vec<Part>) -> Option<(usize, String)> {
    parts.iter().enumerate().find_map(|(i, part)| {
        if let Part::Divert(Some(target)) = part {
            Some((i, target.to_string()))
        } else {
            None
        }
    })
}

fn break_at_break(parts: &Vec<Part>) -> Vec<&[Part]> {
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

crate fn print_segments(segments: &Vec<Segment>) -> TokenStream {
    let mut output = TokenStream::new();
    let mut parts = vec![];
    for segment in segments {
        match segment {
            Segment::Text(message) => {
                parts = [parts, message.parts.clone()].concat();
                if let Some((i, divert)) = find_divert(&parts) {
                    return output;
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
                            parts: [choice.prefix.parts.clone(), choice.suffix.parts.clone()]
                                .concat(),
                        });
                        let tokens = print_segments(&[vec![segment], case.clone()].concat());
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
    for parts in &breaks[0..breaks.len() - 1] {
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
