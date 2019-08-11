use std::io;

use clap::{App, Arg};
use sudoku::Sudoku;

use sudoku_solver::*;

fn main() {
    let matches =
        App::new("sudoku-solver")
                .version("1.0")
                .arg(
                    Arg::with_name("generate").help("Generate a sudoku puzzle to solve.")
                )
                .get_matches();

    let unsolved_puzzle = if matches.is_present("generate") {
        Sudoku::generate_unique()
    } else {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        Sudoku::from_str_line(&input).expect("Input is not a valid puzzle")
    };

    println!("{}", unsolved_puzzle.display_block());
    println!("\n==============");

    let input = format!("{}", unsolved_puzzle.to_str_line());
    let solution = solve_sudoku(&input).expect("Unable to solve puzzle");
    let solved_puzzle = Sudoku::from_str_line(&solution).expect("Unable to parse puzzle");

    println!("{}\n", solved_puzzle.display_block());
    assert!(solved_puzzle.is_solved());
}
