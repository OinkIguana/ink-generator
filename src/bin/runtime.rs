#![feature(generators, generator_trait)]

mod test;

fn main() {
    let mut story = test::story::story();
    loop {
        if let Some((paragraph, s)) = story.next(0) {
            story = s;
            println!("{}", paragraph.text());
            if let Some(choices) = paragraph.choices() {
                for (i, choice) in choices.iter().enumerate() {
                    println!("{}: {}", i, choice);
                }
            }
        } else {
            break;
        }
    }
}
