#![feature(generators, generator_trait, crate_visibility_modifier)]

use std::io::{stdout, Write};

use serde::Deserialize;
use ron::de::from_str;
use colored::*;
use text_io::*;
use inkgen::runtime::Story;

mod story;

#[derive(Deserialize, Debug)]
enum Event {
    Win,
    Lose,
}

fn main() -> Result<(), ron::de::Error> {
    let mut story = story::story::story();
    let mut selection = 0usize;

    let _ended_story = loop {
        let (paragraph, s) = match story {
            Story::Unstarted(story) => story.start(),
            Story::Regular(story) => story.select(selection),
            Story::Ended(story) => break story,
        };
        println!("{}", paragraph.text());
        for tag in paragraph.tags().iter().map(|string| from_str::<Event>(string)) {
            println!("{}", format!("Event: {:?}", tag?).green());
        }
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

    Ok(())
}
