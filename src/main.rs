#[macro_use]
extern crate clap;
extern crate utils;

mod state;
mod grid;
mod node;
mod puzzle_gen;
use clap::{Arg, App};
use std::{path::Path, fs};

fn check_result(input: Vec<u16>, lgth: u8) -> bool
{
    if input == puzzle_gen::summon_snail(lgth)
    {
        return true;
    }
    false
}

fn create_random_grid(lgth: u8) -> grid::Grid
{
    grid::Grid::new(puzzle_gen::random_puzzle(lgth))
}

fn sort_check_and_dedup(mut input: Vec<u16>) -> bool
{
    let len = input.len();
    input.sort();
    input.dedup();
    if input.len() == len && *input.last().unwrap() == len as u16 - 1  && *input.first().unwrap() == 0
    {
        return true
    }
    false
}

fn parser(content: String) -> Result<grid::Grid, String>
{
    let ret: Vec<u16> = content.split(char::is_whitespace).flat_map(|x| x.parse()).collect();
    let sqr_len: u16 = (ret.len() as f64).sqrt() as u16;
    if sqr_len > 2 && sqr_len < 8 && sort_check_and_dedup(ret.clone())
    {
        return Ok(grid::Grid::new(ret))
    }
    println!("{:#?}, sqr = {}", content, sqr_len);
    Err("Invalid puzzle format".into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> 
{
    //println!("{}", grid::Grid::new(puzzle_gen::random_puzzle(7)));
    let matches = App::new("N-Puzzle")
                .version(crate_version!())
                .author(crate_authors!())
                .about(crate_description!())
                .arg(Arg::with_name("input")
                    .required(true)
                    .help("<file.txt> input"))
                .get_matches();

    let content = fs::read_to_string(Path::new(matches.value_of("input").unwrap())).unwrap();
    let grid = parser(content)?;
    println!("{}", grid);

    Ok(())
    // TODO Parser here
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn parsing()
    {
        assert_eq!(parser("0 9 2 3 6 4 5 8 7 1 11 10".into()).unwrap().get_map(), vec![0, 9, 2, 3, 6, 4, 5, 8, 7, 1, 11, 10]);
    }
}