#[macro_use]
extern crate clap;
extern crate utils;

mod state;
mod grid;
mod node;
mod puzzle_gen;
mod algo;
use clap::{Arg, App, AppSettings, ArgSettings};
use std::{path::Path, fs};
use grid::{Grid, HType};
use node::Node;
use state::State;
use algo::{Algo, AType};

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
    let mut content_lines = utils::remove_comment_by_line(&content, "#");
    let mut nb_lines = content_lines.len();

    let nb_col = if nb_lines > 0
    {
        let first = content_lines.remove(0);
        nb_lines -= 1;
        match first.parse::<usize>()
        {
            Ok(col) => col,
            Err(_) => return Err(format!("Invalid puzzle size: [{}]", first))
        }
    }
    else
    {
        return Err(format!("There is no way we can resolve an empty puzzle dummy!"));
    };
    if nb_col != nb_lines
    {
        return Err(format!("The size definition and the number of line of the puzzle don't match"));
    }
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

fn expect_size(nbr: String) -> Result<(), String>
{
    if nbr.parse::<u8>().is_ok()
    {
        if nbr.parse::<u8>().unwrap() > 2 && nbr.parse::<u8>().unwrap() < 9
        {
            return Ok(())
        }
        else
        {
            return Err(String::from("Number must be between 2 and 8"))
        }
    }
    Err(String::from("Expected a number"))
}

fn expect_weight(nbr: String) -> Result<(), String>
{
    if nbr.parse::<u8>().is_ok()
    {
        if nbr.parse::<u8>().unwrap() > 0 && nbr.parse::<u8>().unwrap() < 100
        {
            return Ok(())
        }
        else
        {
            return Err(String::from("Number must be between 0 and 100"))
        }
    }
    Err(String::from("Expected a number"))
}

fn expect_gscore(nbr: String) -> Result<(), String>
{
    if nbr.parse::<u32>().is_ok()
    {
        if nbr.parse::<u32>().unwrap() > 0 && nbr.parse::<u32>().unwrap() < u32::max_value()
        {
            return Ok(())
        }
        else
        {
            return Err(String::from("Number must be between 0 and U32MAX"))
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

fn error_handler<T, E>(from: Result<T, E>) -> T
where E:
    std::fmt::Display
{
    match from
    {
        Ok(res) => res,
        Err(e) => {
            eprintln!("ERROR: {}", e);
            std::process::exit(42);
        }
    }
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
                    .help("Input file. Must be square, solvable, larger than 2 by 2 and smaller than 9 by 9."))
                .arg(Arg::with_name("random")
                    .short("r")
                    .long("random")
                    .number_of_values(1)
                    .validator(expect_size)
                    .help("Generate random grid (between 3 and 8)."))
                .arg(Arg::with_name("heuristic")
                    .short("e")
                    .long("heuristic")
                    .number_of_values(1)
                    .possible_values(&["hamming", 
                                    "manhattan", 
                                    "linear_manhattan"])
                    .help("Choose heuristic model. Default is linear manhattan (fastest)."))
                .arg(Arg::with_name("algorithm")
                    .short("a")
                    .long("algorithm")
                    .number_of_values(1)
                    .possible_values(&["astar", "idastar"])
                    .help("Choose algorithm. Default depends on the submitted grid."))
                .arg(Arg::with_name("weight")
                    .short("w")
                    .long("weight")
                    .number_of_values(1)
                    .validator(expect_weight)
                    .help("Force heuristic max weight to value. Max is 100."))
                .arg(Arg::with_name("uniform")
                    .short("u")
                    .long("uniform")
                    .validator(expect_gscore)
                    .number_of_values(1)
                    .help("Set heuristic model variant to uniform cost with value. Max is u32_max"))
                .arg(Arg::with_name("greedy")
                    .short("g")
                    .long("greedy")
                    .conflicts_with("uniform")
                    .conflicts_with("weight")
                    .takes_value(false)
                    .help("Set heuristic model variant to greedy."))
                .get_matches();

    let grid = if matches.value_of("input").is_some()
    {
        let content = error_handler(fs::read_to_string(Path::new(matches.value_of("input").expect("Invalid input"))));
        error_handler(parser(content))
    }
    else
    {
        let lines = matches.value_of("random").unwrap().parse().unwrap();
        Grid::new(puzzle_gen::random_puzzle(lines), lines)
    };
    println!("{}", grid);
    if !grid.solvable()
    {
        error_handler(Err(String::from("Grid is unsolvable !")))
    }
    let lines = grid.get_lines();
    let greed = matches.is_present("greedy");
    let g_score: u32 = match matches.value_of("uniform")
    {
        Some(x) => x.parse().unwrap(),
        None => 0,
    };
    let max_weight: u8 = match matches.value_of("weight")
    {
        Some(x) => x.parse().unwrap(),
        None => (lines / 2 + 1),
    };
    let a_type = error_handler(AType::from_str_or_default(matches.value_of("algorithm")));
    let h_type = error_handler(HType::from_str_or_default(matches.value_of("heuristic")));
    let mut initial_node = Node::new(State::default(), grid);
    let goal = Grid::new(puzzle_gen::create_snail_goal(lines), lines as u8);
    initial_node.update_state(&goal, h_type);
    let mut algo = Algo::new(initial_node.clone(), goal.clone(), h_type, lines);
    match algo.resolve()
    {
        Some(solution) =>
        {
            println!("A solution was found for the initial state you gave\nHere are the results:\n");
            println!("Amount of moves required:\t{}\n", solution.borrow().state.g);
            println!("Complexity in time:\t\t{}\n(number of nodes processed)\n", algo.get_nb_popped());
            println!("Complexity in size:\t\t{}\n(number of nodes in memory at the same time)\n", algo.get_nb_nodes_wm());
            println!("Steps to reach the goal:");
            solution.borrow().print_steps();
            Ok(())
        },
        None => Err(format!("There is no way the provided n-puzzle can reach the goal:\nInitial state:\n{}Goal state:\n{}", initial_node.grid, goal).into())
    }
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