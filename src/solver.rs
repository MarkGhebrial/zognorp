// use std::error::Error;

use std::{collections::HashSet, fmt::Display, usize};

use crate::{
    puzzle::{Cell, Puzzle, Valid},
    solver::SolverError::InvalidPuzzle,
};

// #[derive(Error)]
pub enum SolverError {
    InvalidPuzzle(Puzzle),

    // Returned when the solver arrives at a puzzle with no solution
    DeadEnd(Puzzle),
}

impl Display for SolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SolverError::*;
        match self {
            InvalidPuzzle(p) => write!(f, "Invalid puzzle.\n{p}"),
            DeadEnd(p) => write!(
                f,
                "Solver reached a dead end (this should not be a user-facing error).\n{p}"
            ),
        }
    }
}

pub fn solve_sudoku(
    puzzle: Puzzle,
    visited_puzzles: &mut HashSet<Puzzle>,
    depth: usize,
) -> Result<Puzzle, SolverError> {
    // Base cases
    if !puzzle.is_valid() {
        return Err(InvalidPuzzle(puzzle));
    }
    if puzzle.is_solved() {
        return Ok(puzzle);
    }

    if depth > 50 {
        println!("Too deep");
        return Err(SolverError::DeadEnd(puzzle));
    }

    // The first element of the tuple is the cell index. The second element is the
    // set of all valid values of the cell
    let all_possibilities: Vec<(usize, HashSet<Cell>)> = puzzle
        .iter_unset_cells()
        .map(|(cell_index, _cell)| (cell_index, puzzle.possibilities(cell_index)))
        // .filter(|(_cell_index, possibilities)| !possibilities.is_empty())
        .collect();

    for (_cell_index, possibilities) in &all_possibilities {
        if possibilities.is_empty() {
            return Err(SolverError::DeadEnd(puzzle));
        }
    }

    println!("Depth: {depth}");
    println!("{}", puzzle);
    for (cell_index, possibilities) in &all_possibilities {
        if possibilities.is_empty() {
            return Err(SolverError::DeadEnd(puzzle));
        }
        println!(
            "Possible moves for cell {}: {:?}",
            cell_index, possibilities
        );
    }
    println!("========");

    // Sort the list of cells from most constrained to least constrained (i.e. least possible valid values to most possible valid values)
    // let all_possibilities = merge_sort(
    //     all_possibilities.as_slice(),
    //     |(_, a): &(usize, HashSet<Cell>), (_, b): &(usize, HashSet<Cell>)| a.len() < b.len(),
    // );
    // println!("Post sort: {:#?}", all_possibilities);

    let aplen = all_possibilities.len();

    // Iterate through all the valid board states
    for (cell_index, cell_possibilities) in all_possibilities {
        for possibility in cell_possibilities {
            let mut new_puzzle = puzzle.clone();
            new_puzzle.set_cell(cell_index, possibility);

            if !visited_puzzles.insert(new_puzzle.clone()) {
                // println!("Puzzle has been visited:\n{new_puzzle}");
                continue;
            }

            // Recursively solve the new puzzle
            match solve_sudoku(new_puzzle, visited_puzzles, depth + 1) {
                Ok(solved_puzzle) => return Ok(solved_puzzle),
                Err(SolverError::DeadEnd(_)) => { /* continue */ }
                Err(e) => panic!("error when attempting to solve puzzle: {}", e),
            }
        }
    }

    println!(
        "Explored all {} possibilities. No solutions found for this puzzle at depth {depth}",
        aplen
    );
    // println!("Deadend\n{puzzle}");

    Err(SolverError::DeadEnd(puzzle))
}
