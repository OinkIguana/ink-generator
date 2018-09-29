use crate::Error;

mod input;
pub(crate) mod schema;

use self::input::Input;
pub use self::schema::Ink;

fn remove_comments(string: String) -> String {
    let mut in_block = false;

    fn remove_comment(in_block: &mut bool, line: &str) -> String {
        if !*in_block {
            if let Some(index) = line.find("//") {
                line[0..index].to_owned()
            } else if let Some(index) = line.find("/*") {
                *in_block = true;
                line[0..index].to_owned()
            } else {
                line.to_owned()
            }
        } else if let Some(index) = line.rfind("*/") {
            *in_block = false;
            remove_comment(in_block, &line[index + 2..])
        } else {
            line.to_owned()
        }
    }

    string
        .lines()
        .map(|line| remove_comment(&mut in_block, line))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn parse(string: String) -> Result<Ink, Error> {
    let string = remove_comments(string);
    Ink::parse(string)
}
