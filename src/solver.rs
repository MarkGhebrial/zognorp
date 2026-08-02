// use std::error::Error;

use std::{fmt::Display, usize};

use crate::{
    bounded_u8::{BoundedU8, UBoundU8},
    grid::{CellPossibilities, Grid},
};

// #[derive(Error)]
pub enum SolverError {
    InvalidPuzzle,

    // Returned when the solver arrives at a puzzle with no solution
    DeadEnd,
}

impl Display for SolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SolverError::*;
        match self {
            InvalidPuzzle => write!(f, "Invalid puzzle.\n"),
            DeadEnd => write!(
                f,
                "Solver reached a dead end (this should not be a user-facing error).\n"
            ),
        }
    }
}

pub fn solve_sudoku(
    grid: &Grid,
    // visited_puzzles: &mut HashSet<[UBoundU8<9>; 81]>,
    depth: usize,
) -> Result<Grid, SolverError> {
    // Base cases
    if !grid.is_valid() {
        return Err(SolverError::InvalidPuzzle);
    }
    if grid.is_solved() {
        return Ok(grid.clone());
    }

    let all_possibilities: Vec<(usize, &CellPossibilities)> = grid
        .possibilities()
        .iter()
        .enumerate()
        .filter(|(cell_index, _)| grid.cells()[*cell_index] == 0) // Filter out cells that've already been set
        .collect();

    // Reject puzzles where any unset cells don't have valid possibilities
    for (_cell_index, cell_possibilities) in all_possibilities.iter() {
        if cell_possibilities.are_all_impossible() {
            // println!("All impossible");
            return Err(SolverError::DeadEnd);
        }
    }

    // Sort the list of possibilities so that we first search the cells with the fewest possibilities
    // let all_possibilities = merge_sort(&all_possibilities, |a, b| *a.1.count() < *b.1.count());

    // Find the cell with the fewest amount of possibilities
    let (best_cell_index, best_cell_possibilities) = all_possibilities
        .iter()
        .min_by(|(_, a), (_, b)| a.count().cmp(&b.count()))
        .unwrap();

    // for (cell_index, cell_possibilities) in all_possibilities {
    for cell_value in (1..=9).map(|n: u8| UBoundU8::<9>::new(n)) {
        // Skip impossible guesses
        if !best_cell_possibilities.is_possible(cell_value) {
            continue;
        }

        //println!("Guessing {} for cell index {} at depth {}", *cell_value, cell_index, depth);

        let mut new_puzzle = grid.clone();
        new_puzzle.set_cell(BoundedU8::new(*best_cell_index as u8), cell_value);

        // if !visited_puzzles.insert(new_puzzle.cells().clone()) {
        //     continue;
        // }

        // Recursively solve the new puzzle
        match solve_sudoku(&new_puzzle, depth + 1) {
            Ok(solved_puzzle) => return Ok(solved_puzzle),
            Err(SolverError::DeadEnd) => { /* continue */ }
            Err(e) => panic!("error when attempting to solve puzzle: {}", e),
        }
    }
    // break;
    // }

    Err(SolverError::DeadEnd)
}
