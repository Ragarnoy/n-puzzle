#[macro_use]
extern crate clap;
extern crate utils;

mod state;
mod grid;
mod node;
mod puzzle_gen;
mod algo;
use clap::{Arg, App, AppSettings};
use std::{path::Path, fs};
use grid::{Grid, HType};
use node::Node;
use state::State;
use algo::Algo;

fn create_random_grid(lgth: u8) -> grid::Grid
{
    grid::Grid::new(puzzle_gen::random_puzzle(lgth), lgth)
}

fn sort_check_and_dedup(mut input: Vec<u16>) -> bool
{
    let len = input.len();
    input.sort();
    input.dedup();
    // The line below could seem weird as there is no `if` but in fact this line already returns a `bool` as expected.
    input.len() == len && *input.last().unwrap() == len as u16 - 1  && *input.first().unwrap() == 0
}

fn parser(content: String) -> Result<grid::Grid, String>
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
        Ok((Grid::new(ret, nb_lines as u8)))
    }
    else
    {
        Err("Invalid puzzle format".into())
    }
}

fn expect_integer(nbr: String) -> Result<(), String>
{
    if nbr.parse::<u8>().is_ok()
    {
        if nbr.parse::<u8>().unwrap() > 2 && nbr.parse::<u8>().unwrap() < 16
        {
            return Ok(())
        }
        else
        {
            return Err(String::from("Number must be between 2 and 16"))
        }
    }
    Err(String::from("Expected a number"))
}

fn expect_file(file: String) -> Result<(), String>
{
    if Path::new(&file).exists()
    {
        if Path::new(&file).is_file()
        {
            return Ok(())
        }
        else
        {
            return Err(String::from("File expected."))
        }
    }
    Err(String::from("Path is invalid/does not exist."))
}

fn main() -> Result<(), Box<dyn std::error::Error>> 
{
    let matches = App::new("N-Puzzle")
                .version(crate_version!())
                .author(crate_authors!())
                .about(crate_description!())
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(Arg::with_name("input")
                    .index(1)
                    .number_of_values(1)
                    .conflicts_with("random")
                    .required_unless("random")
                    .validator(expect_file)
                    .help("<input.txt>"))
                .arg(Arg::with_name("random")
                    .short("r")
                    .help("-r <3-16> (conflicts with file input)")
                    .number_of_values(1)
                    .validator(expect_integer))
                .arg(Arg::with_name("heuristic")
                    .short("e")
                    .required(false)
                    .long("heuristic")
                    .number_of_values(1)
                    .help("<heuristic_name>")
                    .possible_values(&["hamming", 
                                    "manhattan", 
                                    "linear_manhattan"]))
                .get_matches();

    let content: String;
    let grid;
    let lines: u8;
    if matches.value_of("input").is_some()
    {
        content = fs::read_to_string(Path::new(matches.value_of("input").expect("Invalid input")))?;
        grid = parser(content)?;
        lines = grid.get_lines();
    }
    else if matches.value_of("random").is_some()
    {
        lines = matches.value_of("random").unwrap().parse().unwrap();
        grid = Grid::new(puzzle_gen::random_puzzle(lines), lines);
        // TODO Make solvable lol
        println!("{}", grid);
    }
    else
    {
        // impossible
        panic!("bruh wtf");
    }
    let h_type = HType::from_str_or_default(matches.value_of("heuristic"))?;
    let mut initial_node = Node::new(State::default(), grid);
    let goal = Grid::new(puzzle_gen::create_snail_goal(lines), lines as u8);
    initial_node.update_state(&goal, h_type);
    let mut algo = Algo::new(initial_node, goal, h_type, lines);
    let result = algo.resolve();
    println!("{}", result.unwrap().borrow().grid);

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