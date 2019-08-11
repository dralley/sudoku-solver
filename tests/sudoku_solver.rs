use sudoku;
use sudoku_solver;

static PUZZLES: &str = include_str!("puzzles.txt");

#[test]
fn test_top95() {
    for puzzle in PUZZLES.lines() {
        let solved_puzzle = sudoku_solver::solve_sudoku(puzzle).expect("Returned None");
        assert!(sudoku::Sudoku::from_str_line(&solved_puzzle).expect("Unable to parse").is_solved());
    }
}
