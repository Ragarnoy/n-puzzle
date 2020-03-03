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
<<<<<<< HEAD
use grid::{Grid, HType};
use node::Node;
use state::State;
use algo::Algo;
=======

fn check_result(input: Vec<u16>, lgth: u8) -> bool
{
    if input == puzzle_gen::create_snail_goal(lgth)
    {
        return true;
    }
    false
}
>>>>>>> feature/snail_sort

fn create_random_grid(lgth: u8) -> grid::Grid
{
    grid::Grid::new(puzzle_gen::random_puzzle(lgth))
}

fn sort_check_and_dedup(mut input: Vec<u16>) -> bool
{
    let len = input.len();
    input.sort();
    input.dedup();
    // The line below could seem weird as there is no `if` but in fact this line already returns a `bool` as expected.
    input.len() == len && *input.last().unwrap() == len as u16 - 1  && *input.first().unwrap() == 0
}

fn parser(content: String) -> Result<(u8, grid::Grid), String>
{
    let mut ret: Vec<u16> = Vec::new();
    let content_lines = utils::remove_comment_by_line(&content, "#");
    let nb_lines = content_lines.len();

    for line in content_lines
    {
        let mut invalid_token = false;
        println!("DEBUG::main::parser: current line: {}", line);
        let mut parsed_line: Vec<u16> = line.split_whitespace().map(|x| {
            let res = x.parse::<u16>();
            if res.is_err()
            {
                invalid_token = true;
            }
            println!("DEBUG::main::parser: current token in line: [{}]", x);
            res.unwrap_or(0)
        }).collect();
        if invalid_token
        {
            return Err(format!("At least one invalid token found in the following line: {}", line));
        }
        else if parsed_line.len() != nb_lines
        {
            return Err(format!("Invalid puzzle format: we have {} lines but the following line contains {} columns: {}", nb_lines, parsed_line.len(), line));
        }
        ret.append(&mut parsed_line);
    }
    if sort_check_and_dedup(ret.clone())
    {
        // No need to clone `ret` here because it will be dropped at the end
        // of this function so we can safely give ownership to the new `Grid`.
        Ok((nb_lines as u8, Grid::new(ret)))
    }
    else
    {
        Err("Invalid puzzle format".into())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> 
{
    let matches = App::new("N-Puzzle")
                .version(crate_version!())
                .author(crate_authors!())
                .about(crate_description!())
                .arg(Arg::with_name("input")
                    .conflicts_with("random")
                    .help("<file.txt> input"))
                .arg(Arg::with_name("heuristic")
                    .required(false)
                    .multiple(false)
                    .help("<heuristic_name>"))
                .get_matches();

    let content = fs::read_to_string(Path::new(matches.value_of("input").unwrap_or("")))?;
    let h_type = HType::from_str_or_default(matches.value_of("heuristic"))?;
    let (nb_col, grid) = parser(content)?;
    let mut initial_node = Node::new(State::default(), grid);
    let goal = Grid::new(puzzle_gen::summon_snail(nb_col));
    initial_node.update_state(&goal, h_type, nb_col);
    let mut algo = Algo::new(initial_node, goal, h_type, nb_col);
    let result = algo.resolve();
    println!("{:#?}", result);

    Ok(())
}


/* #[cfg(test)]
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
        assert_eq!(parser(test_str).unwrap().1.get_map(), test_vec);
    }

    #[test]
    fn parsing_pass_four()
    {
        let mut rng = thread_rng();
        let mut test_vec: Vec<u16> = (0..16).collect();
        test_vec.shuffle(&mut rng);
        let test_str: String = test_vec.iter().map(|x| x.to_string() + " ").collect();
        assert_eq!(parser(test_str).unwrap().1.get_map(), test_vec);
    }

    #[test]
    fn parsing_pass_five()
    {
        let mut rng = thread_rng();
        let mut test_vec: Vec<u16> = (0..25).collect();
        test_vec.shuffle(&mut rng);
        let test_str: String = test_vec.iter().map(|x| x.to_string() + " ").collect();
        assert_eq!(parser(test_str).unwrap().1.get_map(), test_vec);
    }

    #[test]
    fn parsing_pass_six()
    {
        let mut rng = thread_rng();
        let mut test_vec: Vec<u16> = (0..36).collect();
        test_vec.shuffle(&mut rng);
        let test_str: String = test_vec.iter().map(|x| x.to_string() + " ").collect();
        assert_eq!(parser(test_str).unwrap().1.get_map(), test_vec);
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
} */