// use std::error::Error;

use std::{collections::HashSet, fmt::Display, usize};

use crate::{
    puzzle::{self, Cell, Puzzle},
    sort::merge_sort,
};

// #[derive(Error)]
pub enum SolverError {
    InvalidRow(usize),
    InvalidColumn(usize),
    InvalidBlock(usize),
    DeadEnd(Puzzle),
}

impl Display for SolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SolverError::*;
        match self {
            InvalidRow(i) => write!(f, "Row {} of the puzzle is not valid!", i),
            InvalidColumn(i) => write!(f, "Column {} of the puzzle is not valid!", i),
            InvalidBlock(i) => write!(f, "Block {} of the puzzle is not valid!", i),
            DeadEnd(_) => write!(
                f,
                "Solver reached a dead end (this should not be a user-facing error)"
            ),
        }
    }
}

pub fn solve_sudoku(puzzle: Puzzle) -> Result<Puzzle, SolverError> {
    if puzzle.is_solved() {
        return Ok(puzzle);
    }

    // TODO: Check if the puzzle is valid.

    // The first element of the tuple is the cell index. The second element is the
    // set of all valid values of the cell
    let all_possibilities: Vec<(usize, HashSet<Cell>)> = puzzle
        .iter_unset_cells()
        .map(|(cell_index, _cell)| (cell_index, puzzle.possibilities(cell_index)))
        .collect();

    // Sort the list of cells from most constrained to least constrained (i.e. least possible valid values to most possible valid values)
    let all_possibilities = merge_sort(
        all_possibilities.as_slice(),
        |(_, a): &(usize, HashSet<Cell>), (_, b): &(usize, HashSet<Cell>)| a.len() < b.len(),
    );
    println!("Post sort: {:#?}", all_possibilities);

    // Iterate through all the valid board states, starting with the ones
    for (cell_index, cell_possibilities) in all_possibilities {
        for possibility in cell_possibilities {
            let new_puzzle = puzzle.set_cell(cell_index, possibility);

            // Recursively solve the new puzzle
            match solve_sudoku(new_puzzle) {
                Ok(solved_puzzle) => return Ok(solved_puzzle),
                Err(SolverError::DeadEnd(_)) => { /* continue */ }
                Err(e) => panic!("error when attempting to solve puzzle: {}", e),
            }
        }
    }

    Err(SolverError::DeadEnd(puzzle))
}
