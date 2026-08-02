use crate::{
    bounded_u8::UBoundU8,
    grid::Grid,
    solver::solve_sudoku,
};

mod bounded_u8;
mod grid;
mod puzzle;
mod solver;
mod sort;

fn main() {
    #[rustfmt::skip]
    let grid: [UBoundU8<9>; 81] = 
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
        .map(|c: u8| UBoundU8::new(c));

    let grid = Grid::new(grid);

    // println!("{}", grid);

    // for (cell_index, possibilities) in grid.possibilities().iter().enumerate() {
    //     // Skip set cells
    //     if *grid.cells()[cell_index] != 0 {
    //         continue;
    //     }

    //     print!("Possibilities for cell {}: ", cell_index);
    //     for i in 1..9 {
    //         if possibilities.is_possible(BoundedU8::new(i)) {
    //             print!("{i}, ");
    //         }
    //     }
    //     println!();
    // }

    match solve_sudoku(&grid, 0) {
        Ok(p) => println!("Found a solution for the first puzzle!\n{p}"),
        Err(e) => println!("{}", e),
    }

    #[rustfmt::skip]
    let grid: [UBoundU8<9>; 81] = 
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
        .map(|c: u8| UBoundU8::new(c));

    let grid = Grid::new(grid);
    match solve_sudoku(&grid, 0) {
        Ok(p) => println!("Found a solution for the second puzzle!\n{p}"),
        Err(e) => println!("{}", e),
    }

    // let mut grid: [Cell; 81] = [Cell::Unset; 81];

    #[rustfmt::skip]
    let grid: [UBoundU8<9>; 81] = 
    [
        0, 0, 0, 2, 0, 0, 6, 5, 0,
        7, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 6, 0, 0, 0, 9, 0, 0, 0,
        6, 1, 0, 7, 0, 0, 0, 9, 0,
        0, 0, 7, 0, 1, 0, 0, 0, 5,
        4, 0, 0, 0, 0, 5, 0, 0, 0,
        3, 0, 0, 0, 0, 0, 1, 0, 0,
        0, 0, 0, 9, 2, 8, 0, 0, 4,
        0, 0, 0, 0, 0, 0, 0, 2, 8
    ]
        .map(|c: u8| UBoundU8::new(c));

    let grid = Grid::new(grid);

    match solve_sudoku(&grid, 0) {
        Ok(p) => println!("Found a solution to the third puzzle!\n{p}"),
        Err(e) => println!("{}", e),
    }
}
