use crate::{
    bounded_u8::{BoundedU8, UBoundU8},
    grid::{CellPossibilities, Grid},
};


pub fn solve_sudoku(grid: &Grid) -> Option<Grid> {
    // Base cases
    if !grid.is_valid() {
        return None;
    }
    if grid.is_solved() {
        return Some(grid.clone());
    }

    let all_possibilities: Vec<(usize, &CellPossibilities)> = grid
        .possibilities()
        .iter()
        .enumerate()
        .filter(|(cell_index, _)| grid.cells()[*cell_index] == 0) // Filter out cells that've already been set
        .collect();


    // Find the cell with the fewest amount of possibilities
    let (best_cell_index, best_cell_possibilities) = all_possibilities
        .iter()
        .min_by_key(|(_, possibilities)| possibilities.count())
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
            Some(solved_puzzle) => return Some(solved_puzzle),
            None => { /* continue */ }
        }
    }

    // No solutions found
    None
}
