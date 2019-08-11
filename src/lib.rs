
use varisat;
use varisat::ExtendFormula;

pub fn solve_sudoku(problem: &str) -> Option<String> {
    let mut solver = varisat::Solver::new();
    let mut matrix: Vec<varisat::Lit> = Vec::new();

    // Create literals inside the solver
    for _row in 0..9 {
        for _column in 0..9 {
            for _poss_value in 0..9 {
                matrix.push(solver.new_lit());
            }
        }
    }

    // Rule 1: no row contains duplicate numbers
    for row in 0..9 {
        for poss_value in 0..9 {
            let mut lits = Vec::new();
            for column in 0..9 {
                lits.push(matrix[get_lit_idx(row, column, poss_value)]);
            }
            exactly_one_true(&mut solver, &lits);
        }
    }

    // Rule 2: no column contains duplicate numbers
    for column in 0..9 {
        for poss_value in 0..9 {
            let mut lits = Vec::new();
            for row in 0..9 {
                lits.push(matrix[get_lit_idx(row, column, poss_value)]);
            }
            exactly_one_true(&mut solver, &lits);
        }
    }

    // Rule 3: no 3x3 grid contains duplicate numbers
    for block_row in (0..9).step_by(3) {
        for block_column in (0..9).step_by(3) {
            for poss_value in 0..9 {
                let mut lits = Vec::new();
                for sub_block_row in 0..3 {
                    for sub_block_column in 0..3 {
                        let idx = get_lit_idx(block_row + sub_block_row,
                                              block_column + sub_block_column,
                                              poss_value);
                        lits.push(matrix[idx]);
                    }
                }
                exactly_one_true(&mut solver, &lits);
            }
        }
    }

    // Rule 4: each cell contains exactly one number
    for row in 0..9 {
        for column in 0..9 {
            let mut lits = Vec::new();
            for poss_value in 0..9 {
                lits.push(matrix[get_lit_idx(row, column, poss_value)]);
            }
            exactly_one_true(&mut solver, &lits);
        }
    }

    // Create rules for known values
    let slots = problem.chars().map(|c| c.to_digit(10)).enumerate();
    for (idx, val) in slots {
        if let Some(val) = val {
            solver.add_clause(&[matrix[idx * 9 + (val as usize) - 1]]);
        }
    }

    // Solve
    solver.solve().unwrap();
    let model = solver.model().unwrap();

    // Convert the solution into a string
    let mut result = String::new();
    for row in 0..9 {
        for column in 0..9 {
            for poss_value in 0..9 {
                if model[get_lit_idx(row, column, poss_value)].is_positive() {
                    result.push_str(&format!("{}", poss_value + 1));
                }
            }
        }
    }
    Some(result)
}

const fn get_lit_idx(row: usize, column: usize, poss_value: usize) -> usize {
    row * 9 * 9 + column * 9 + poss_value
}

fn exactly_one_true(solver: &mut varisat::Solver, lits: &[varisat::Lit]) {
    solver.add_clause(lits);
    for i in 0..lits.len() {
        for j in i+1..lits.len() {
             solver.add_clause(&[!lits[i], !lits[j]]);
        }
    }
}
