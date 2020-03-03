#[macro_use]
extern crate clap;
extern crate utils;

mod state;
mod grid;
mod node;
mod puzzle_gen;
mod algo;
use clap::{Arg, App};
use std::{path::Path, fs};

fn check_result(input: Vec<u16>, lgth: u8) -> bool
{
    if input == puzzle_gen::create_snail_goal(lgth)
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
    let sqr_len = (ret.len() as f64).sqrt();
    if sqr_len.fract() == 0.0 && sqr_len > 2.0 && sqr_len < 8.0 && sort_check_and_dedup(ret.clone())
    {
        println!("{}",grid::Grid::new(ret.clone()));
        return Ok(grid::Grid::new(ret))
    }
    Err("Invalid puzzle format".into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> 
{
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
}


#[cfg(test)]
mod tests
{
    use super::*;
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    #[test]
    fn parsing_pass_three()
    {
        let mut rng = thread_rng();
        let mut test_vec: Vec<u16> = (0..9).collect();
        test_vec.shuffle(&mut rng);
        let test_str: String = test_vec.iter().map(|x| x.to_string() + " ").collect();
        assert_eq!(parser(test_str).unwrap().get_map(), test_vec);
    }

    #[test]
    fn parsing_pass_four()
    {
        let mut rng = thread_rng();
        let mut test_vec: Vec<u16> = (0..16).collect();
        test_vec.shuffle(&mut rng);
        let test_str: String = test_vec.iter().map(|x| x.to_string() + " ").collect();
        assert_eq!(parser(test_str).unwrap().get_map(), test_vec);
    }

    #[test]
    fn parsing_pass_five()
    {
        let mut rng = thread_rng();
        let mut test_vec: Vec<u16> = (0..25).collect();
        test_vec.shuffle(&mut rng);
        let test_str: String = test_vec.iter().map(|x| x.to_string() + " ").collect();
        assert_eq!(parser(test_str).unwrap().get_map(), test_vec);
    }

    #[test]
    fn parsing_pass_six()
    {
        let mut rng = thread_rng();
        let mut test_vec: Vec<u16> = (0..36).collect();
        test_vec.shuffle(&mut rng);
        let test_str: String = test_vec.iter().map(|x| x.to_string() + " ").collect();
        assert_eq!(parser(test_str).unwrap().get_map(), test_vec);
    }

    #[test]
    fn parsing_fail_length()
    {
        let test_vec: Vec<u16> = (0..37).collect();
        let test_str: String = test_vec.iter().map(|x| x.to_string() + " ").collect();
        assert_eq!(parser(test_str), Err("Invalid puzzle format".into()));
    }

    #[test]
    fn parsing_fail_char()
    {
        let mut test_str: String = "0 1 2 3 4 5 6 7 W 9".into();
        assert_eq!(parser(test_str), Err("Invalid puzzle format".into()));
        test_str = "D W 2 D 4 F 6 7 W 9".into();
        assert_eq!(parser(test_str), Err("Invalid puzzle format".into()));
        test_str = "0 1 2 3 4 5 6 7 8 9 W Q A X C W".into();
        assert_eq!(parser(test_str), Err("Invalid puzzle format".into()));
    }
}