use std::fmt::Display;

use crate::{bounded_u8::UBoundU8, puzzle::Puzzle};

#[derive(Clone, Copy)]
pub struct CellPossibilities {
    /// Each bit corresponds with whether or not the corresponding cell value is possible. Since we only
    /// nave 8 bits of space, the nth bit corresponds to digit n+1. i.e bit zero corresponds to the cell
    /// being able to be set to one.
    inner: u16,
}

impl CellPossibilities {
    /// Creates a new CellPossibilities with every possibility set
    pub fn new() -> Self {
        Self { inner: u16::MAX }
    }

    /// Is the given value possible?
    pub fn is_possible(&self, value: UBoundU8<9>) -> bool {
        if value == 0 {
            return false;
        }
        // Check the n-1th bit!
        (self.inner & (1 << (*value - 1))) != 0
    }

    /// Set the given value to be impossible
    pub fn set_impossible(&mut self, value: UBoundU8<9>) {
        if value == 0 {
            return;
        }
        self.inner &= !(1 << (*value - 1));
    }

    pub fn set_all_impossible(&mut self) {
        self.inner = 0;
    }
}

pub struct Grid {
    cells: [UBoundU8<9>; 81],
    possibilities: [CellPossibilities; 81],
}

impl Grid {
    pub fn new(cells: [UBoundU8<9>; 81]) -> Self {
        let mut possibilities = [CellPossibilities::new(); 81];

        // For each cell
        for (cell_index, cell_value) in cells.iter().enumerate() {
            // Skip calculating the possibilities for cells that're already set
            if *cell_value != 0 {
                possibilities[cell_index].set_all_impossible();
                continue;
            }

            // Scan through the cell's neighbors
            for neighbor_index in Puzzle::neighbor_indices(cell_index) {
                println!("Neighbor {}", neighbor_index);
                let neighbor_value = &cells[neighbor_index];
                // Is the neighbor set?
                if *neighbor_value != 0 {
                    println!("Setting impossibility");
                    // If so, call .set_impossible(neighbor's value) on the cell's possibilities
                    possibilities[cell_index].set_impossible(*neighbor_value);
                }
            }
        }

        Self {
            cells,
            possibilities,
        }
    }

    pub fn cells(&self) -> &[UBoundU8<9>; 81] {
        &self.cells
    }

    pub fn possibilities(&self) -> &[CellPossibilities; 81] {
        &self.possibilities
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..=8 {
            // Print the horizontal separators between blocks
            if row == 3 || row == 6 {
                write!(f, "---------------------\n")?;
            }

            for col in 0..9 {
                // Print the vertical separators between blocks
                if col == 3 || col == 6 {
                    write!(f, "| ")?;
                }

                write!(f, "{} ", *self.cells[row * 9 + col])?
            }

            write!(f, "\n")?
        }
        Ok(())
    }
}
