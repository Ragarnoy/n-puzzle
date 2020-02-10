#[macro_use]
extern crate clap;
extern crate utils;
extern crate rand;

mod state;
mod grid;
mod node;
use clap::{Arg, App};
use std::{path::Path, fs};
use rand::seq::SliceRandom;
use rand::thread_rng;

fn summon_snail(lgth: u8) -> Vec<u16>
{
    if lgth == 3
    {
        vec![1, 2, 3,
             8, 0, 4,
             7, 6, 5]
    }
    else if lgth == 4
    {
        vec![1, 2, 3, 4,
             12, 13, 14, 5,
             11, 0, 15, 6,
             10, 9, 8, 7]
    }
    else if lgth == 5
    {
        vec![1, 2, 3, 4, 5,
             16, 17, 18, 19, 6,
             15, 24, 0, 20, 7,
             14, 23, 22, 21, 8,
             13, 12, 11, 10, 9]
    }
    else if lgth == 6
    {
        vec![1, 2, 3, 4, 5, 6,
             20, 21, 22, 23, 24, 7,
             19, 32, 33, 34, 25, 8,
              18, 31, 0, 35, 26, 9,
             17, 30, 29, 28, 27, 10,
             16, 15, 14, 13, 12, 11]
    }
    else if lgth == 7
    {
        vec![1, 2, 3, 4, 5, 6, 7,
            24, 25, 26, 27, 28, 29, 8,
            23, 40, 41, 42, 43, 30, 9,
            22, 39, 48, 0, 44, 31, 10,
            21, 38, 47, 46, 45, 32, 11,
            20, 37, 36, 35, 34, 33, 12,
            19, 28, 17, 16, 15, 14, 13]
    }
    else 
    {
        vec![0]
    }
}

fn random_puzzle(input: u8) -> Vec<u16>
{
    let mut rng = thread_rng();
    let mut snail = summon_snail(input);
    println!("{:#?}", snail);
    snail.shuffle(&mut rng);
    println!("{:#?}", snail);
    snail
}

fn main() {
    random_puzzle(5);
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