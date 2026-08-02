use std::collections::HashSet;

use crate::{
    bounded_u8::BoundedU8,
    puzzle::{Cell, Puzzle},
    grid::{Grid},
    solver::solve_sudoku,
};

mod bounded_u8;
mod grid_index;
mod puzzle;
mod grid;
mod solver;
mod sort;

fn main() {
    #[rustfmt::skip]
    let grid: [u8; 81] = 
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
    ];

    let grid = Grid::new(grid.map(|c: u8| BoundedU8::new(c)));

    // proper_grid.pretty_print(std::io::stdout()).unwrap();
    println!("{}", grid);

    for (cell_index, possibilities) in grid.possibilities().iter().enumerate() {
        // Skip set cells
        if *grid.cells()[cell_index] != 0 {
            continue;
        }

        print!("Possibilities for cell {}: ", cell_index);
        for i in 1..9 {
            if possibilities.is_possible(BoundedU8::new(i)) {
                print!("{i}, ");
            }
        }
        println!();
    }

    // let cell_grid = grid
    //     .map(|c: u8| Cell::from(c));

    // let puzzle = Puzzle::new(grid);

    // match solve_sudoku(puzzle, &mut HashSet::new(), 0) {
    //     Ok(p) => println!("Found a solution for the first puzzle!\n{p}"),
    //     Err(e) => println!("{}", e),
    // }

    // #[rustfmt::skip]
    // [
    //     2, 0, 0, 0, 1, 0, 3, 6, 0,
    //     1, 0, 0, 9, 0, 0, 5, 0, 0,
    //     0, 0, 3, 0, 0, 4, 0, 0, 0,
    //     0, 1, 0, 3, 7, 0, 0, 0, 0,
    //     7, 0, 0, 0, 0, 0, 0, 0, 6,
    //     0, 5, 4, 1, 0, 0, 0, 0, 0,
    //     0, 0, 7, 0, 0, 3, 0, 5, 0,
    //     9, 0, 0, 0, 6, 0, 0, 0, 0,
    //     0, 8, 0, 0, 4, 9, 0, 0, 0,
    // ]
    //     .iter()
    //     .map(|c| Cell::from(*c))
    //     .enumerate()
    //     .for_each(|(i, c)| grid[i] = c);

    // let puzzle = Puzzle::new(grid);

    // match solve_sudoku(puzzle, &mut HashSet::new(), 0) {
    //     Ok(p) => println!("Found a solution for the second puzzle!\n{p}"),
    //     Err(e) => println!("{}", e),
    // }

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
