#![feature(generators, generator_trait)]

use colored::*;
use std::io::{stdout, Write};
use text_io::*;
mod story;

fn main() {
    let mut story = story::story::story();
    let mut selection = 0usize;
    loop {
        let (paragraph, s) = unsafe { story.select(selection) };
        println!("{}", paragraph.text());
        if let Some(choices) = paragraph.choices() {
            println!("");
            for (i, choice) in choices.iter().enumerate() {
                println!("{}", format!("{}: {}", i + 1, choice).blue());
            }
            print!("{}", "?> ".blue());
            stdout().flush().unwrap();
            selection = read!();
        }
        if let Some(s) = s {
            story = s;
        } else {
            break;
        }
    }
}
