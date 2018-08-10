#![feature(generators, generator_trait)]

use colored::*;
use std::io::{stdout, Write};
use text_io::*;
use inkgen::runtime::Story;

mod story;

fn main() {
    let mut story = story::story::story();
    let mut selection = 0usize;
    let _ended_story = loop {
        let (paragraph, s) = match story {
            Story::Unstarted(story) => story.start(),
            Story::Regular(story) => story.select(selection),
            Story::Ended(story) => break story,
        };
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
        story = s;
    };
}
