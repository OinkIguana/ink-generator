#![feature(generators, generator_trait)]

use colored::*;
use text_io::*;
use std::io::{Write, stdout};
mod test;

fn main() {
    let mut story = test::story::story();
    let mut selection = 0usize;
    loop {
        if let Some((paragraph, s)) = story.next(selection) {
            story = s;
            println!("{}", paragraph.text());
            if let Some(choices) = paragraph.choices() {
                for (i, choice) in choices.iter().enumerate() {
                    println!("{}", format!("{}: {}", i, choice).blue());
                }
                print!("{}", "?> ".blue());
                stdout().flush().unwrap();
                selection = read!();
            }
        } else {
            break;
        }
    }
}
