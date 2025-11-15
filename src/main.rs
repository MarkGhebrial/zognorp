use crate::{
    puzzle::{Cell, Puzzle},
    solver::solve_sudoku,
};

mod puzzle;
mod solver;
mod sort;

fn main() {
    let mut grid: [Cell; 81] = [Cell::Unset; 81];

    #[rustfmt::skip]
    [
        5, 3, 0, 0, 7, 0, 0, 0, 0,
        6, 0, 0, 1, 9, 5, 0, 0, 0,
        0, 9, 8, 0, 0, 0, 0, 6, 0,
        8, 0, 0, 0, 6, 0, 0, 0, 3,
        4, 0, 0, 8, 0, 3, 0, 0, 1,
        7, 0, 0, 0, 2, 0, 0, 0, 6,
        0, 6, 0, 0, 0, 0, 2, 8, 0,
        0, 0, 0, 4, 1, 9, 0, 0, 5,
        0, 0, 0, 0, 8, 0, 0, 7, 9 
    ]
        .iter()
        .map(|c| Cell::from(*c))
        .enumerate()
        .for_each(|(i, c)| grid[i] = c);

    println!("67: {:?}", grid[67]);

    let puzzle = Puzzle::new(grid);

    match solve_sudoku(puzzle) {
        Ok(p) => println!("Found a solution!"),
        Err(e) => println!("{}", e),
    }
}
