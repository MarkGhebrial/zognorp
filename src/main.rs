use std::collections::HashSet;

use crate::{
    puzzle::{Cell, Puzzle},
    solver::solve_sudoku,
};

mod puzzle;
mod solver;
mod sort;

fn main() {
    #[rustfmt::skip]
    let mut grid: [Cell; 81] = 
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
        .map(|c: u8| Cell::from(c));

    let puzzle = Puzzle::new(grid);

    match solve_sudoku(puzzle, &mut HashSet::new(), 0) {
        Ok(p) => println!("Found a solution for the first puzzle!\n{p}"),
        Err(e) => println!("{}", e),
    }

    #[rustfmt::skip]
    [
        2, 0, 0, 0, 1, 0, 3, 6, 0,
        1, 0, 0, 9, 0, 0, 5, 0, 0,
        0, 0, 3, 0, 0, 4, 0, 0, 0,
        0, 1, 0, 3, 7, 0, 0, 0, 0,
        7, 0, 0, 0, 0, 0, 0, 0, 6,
        0, 5, 4, 1, 0, 0, 0, 0, 0,
        0, 0, 7, 0, 0, 3, 0, 5, 0,
        9, 0, 0, 0, 6, 0, 0, 0, 0,
        0, 8, 0, 0, 4, 9, 0, 0, 0,
    ]
        .iter()
        .map(|c| Cell::from(*c))
        .enumerate()
        .for_each(|(i, c)| grid[i] = c);

    let puzzle = Puzzle::new(grid);

    match solve_sudoku(puzzle, &mut HashSet::new(), 0) {
        Ok(p) => println!("Found a solution for the second puzzle!\n{p}"),
        Err(e) => println!("{}", e),
    }

    // let mut grid: [Cell; 81] = [Cell::Unset; 81];

    // #[rustfmt::skip]
    // [
    //     0, 0, 0, 2, 0, 0, 6, 5, 0,
    //     7, 0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 6, 0, 0, 0, 9, 0, 0, 0,
    //     6, 1, 0, 7, 0, 0, 0, 9, 0,
    //     0, 0, 7, 0, 1, 0, 0, 0, 5,
    //     4, 0, 0, 0, 0, 5, 0, 0, 0,
    //     3, 0, 0, 0, 0, 0, 1, 0, 0,
    //     0, 0, 0, 9, 2, 8, 0, 0, 4,
    //     0, 0, 0, 0, 0, 0, 0, 2, 8
    // ]
    //     .iter()
    //     .map(|c| Cell::from(*c))
    //     .enumerate()
    //     .for_each(|(i, c)| grid[i] = c);

    // let puzzle = Puzzle::new(grid);

    // match solve_sudoku(puzzle, &mut HashSet::new()) {
    //     Ok(p) => println!("Found a solution!\n{p}"),
    //     Err(e) => println!("{}", e),
    // }
}
