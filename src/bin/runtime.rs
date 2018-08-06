#![feature(generators, generator_trait)]

use colored::*;
use std::io::{stdout, Write};
use text_io::*;
mod story;

fn main() {
    let mut story = story::story::story();
    let mut selection = 0usize;
    loop {
        if let Some((paragraph, s)) = unsafe { story.select(selection) } {
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
