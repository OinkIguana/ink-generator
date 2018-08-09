use inkgen::{parse, pretty_print};
use std::env::args;
use std::fs::File;
use std::io::{stdin, Read, Write};
use std::path::PathBuf;

fn main() {
    let mut string = String::new();
    let infile = args().nth(1).map(PathBuf::from);
    let outfile = args().nth(2).map(PathBuf::from);
    if let Some(mut infile) = infile
        .as_ref()
        .map(|path| File::open(path).expect("Error opening input file"))
    {
        infile
            .read_to_string(&mut string)
            .expect("The input did not contain valid UTF-8");
    } else {
        stdin()
            .read_to_string(&mut string)
            .expect("The input did not contain valid UTF-8");
    }
    let ink = parse(string).unwrap();
    let generated = pretty_print(
        outfile
            .as_ref()
            .and_then(|path| path.file_stem()?.to_str())
            .or(infile.as_ref().and_then(|path| path.file_stem()?.to_str()))
            .unwrap_or("story"),
        ink,
    );
    if let Some(mut outfile) = outfile.map(|path| File::create(path).expect("Error opening input file")) {
        writeln!(outfile, "{}", generated);
    } else {
        println!("{}", generated);
    }
}
