#[macro_use]
extern crate clap;
extern crate utils;

mod state;
mod grid;
mod node;
mod puzzle_gen;
use clap::{Arg, App};
use std::{path::Path, fs};

fn check_result(input: Vec<u16>, lght: u8) -> bool
{
    if input == puzzle_gen::summon_snail(lght)
    {
        return true;
    }
    false
}

fn main() {
    println!("{}", grid::Grid::new(puzzle_gen::random_puzzle(7)));
    //     let matches = App::new("N-Puzzle")
    //                 .version(crate_version!())
    //                 .author(crate_authors!())
    //                 .about(crate_description!())
    //                 .arg(Arg::with_name("input")
    //                     .required(true)
    //                     .help("<file.txt> input"))
    //                 .get_matches();

    // let file = Path::new(matches.value_of("input").unwrap());

    // let content = fs::read_to_string(file).unwrap();
    // println!("{}", content);

    // TODO Parser here
}