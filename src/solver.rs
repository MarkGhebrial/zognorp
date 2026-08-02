use std::fmt::Display;

use crate::{
    bounded_u8::{BoundedU8, UBoundU8},
    grid::{CellPossibilities, Grid},
};

pub enum SolverError {
    InvalidPuzzle,

    // Returned when the solver arrives at a puzzle with no solution
    DeadEnd,
}

impl Display for SolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SolverError::*;
        match self {
            InvalidPuzzle => write!(f, "Invalid puzzle."),
            DeadEnd => write!(
                f,
                "Solver reached a dead end (this should not be a user-facing error)."
            ),
        }
    }
}

pub fn solve_sudoku(grid: &Grid) -> Result<Grid, SolverError> {
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

    // Find the cell with the fewest amount of possibilities
    let (best_cell_index, best_cell_possibilities) = all_possibilities
        .iter()
        .min_by(|(_, a), (_, b)| a.count().cmp(&b.count()))
        .unwrap();

    // Try every number possible value for the cell with the fewest possibilities
    for cell_value in (1..=9)
        .map(|n: u8| UBoundU8::<9>::new(n))
        .filter(|n| best_cell_possibilities.is_possible(*n))
    {
        let mut new_puzzle = grid.clone();
        new_puzzle.set_cell(BoundedU8::new(*best_cell_index as u8), cell_value);

        // Recursively solve the new puzzle
        match solve_sudoku(&new_puzzle) {
            Ok(solved_puzzle) => return Ok(solved_puzzle),
            Err(SolverError::DeadEnd) => { /* continue */ }
            Err(e) => panic!("error when attempting to solve puzzle: {}", e),
        }
    }

    Err(SolverError::DeadEnd)
}
