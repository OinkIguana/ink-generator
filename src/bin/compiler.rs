use inkgen::{parse, pretty_print};
use std::io::{stdin, Read};

fn main() {
    let mut string = String::new();
    stdin()
        .read_to_string(&mut string)
        .expect("The input did not contain valid UTF-8");
    let ink = parse(string).unwrap();
    let string = pretty_print("story", ink);
    println!("{}", string);
}
