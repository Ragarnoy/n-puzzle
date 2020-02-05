#[macro_use]
extern crate clap;

use clap::{Arg, App};
use std::{path::Path, fs, io::Error};



fn main() {
        let matches = App::new("N-Puzzle")
                    .version(crate_version!())
                    .author(crate_authors!())
                    .about(crate_description!())
                    .arg(Arg::with_name("input")
                        .required(true)
                        .help("<file.txt> input"))
                    .get_matches();

    let file = Path::new(matches.value_of("input").unwrap());

    let content = fs::read_to_string(file).unwrap();
    println!("{}", content);

    // TODO Parser here

    let input: Vec<Vec<u32>> = vec![
                        vec![1,2,3],
                        vec![4,0,6],
                        vec![7,8,9]];
}
