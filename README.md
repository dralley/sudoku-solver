A sudoku solver based on the varisat boolean SAT solver

Code based on the blog post [Modern SAT solvers: fast, neat and underused (part 1 of N)](https://codingnest.com/modern-sat-solvers-fast-neat-underused-part-1-of-n/). 

If run with no arguments, it will read a sudoku puzzle from stdin as an 81 character string left-to-right, top-to-bottom with period characters substited for empty spaces. If run with the "generate" argument, the ```sudoku``` crate is used to generate a random sudoku puzzle to solve.
