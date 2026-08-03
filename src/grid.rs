use std::fmt::Display;

use crate::{
    bounded_u8::UBoundU8,
    puzzle::{NEIGHBOR_INDICES},
};

#[derive(Clone, Copy)]
pub struct CellPossibilities {
    /// Each bit corresponds with whether or not the corresponding cell value is possible. Since we only
    /// nave 8 bits of space, the nth bit corresponds to digit n+1. i.e bit zero corresponds to the cell
    /// being able to be set to one.
    inner: u16,
}

impl CellPossibilities {
    /// Nine ones
    const ALL_MASK: u16 = 0x1FF;

    /// Creates a new CellPossibilities with every possibility set
    pub fn new() -> Self {
        Self {
            inner: Self::ALL_MASK,
        }
    }

    /// Is the given value possible?
    pub fn is_possible(&self, value: UBoundU8<9>) -> bool {
        if value == 0 {
            return false;
        }
        // Check the n-1th bit!
        (self.inner & (1 << (*value - 1))) != 0
    }

    pub fn has_possibilities(&self) -> bool {
        (self.inner & Self::ALL_MASK) != 0
    }

    pub fn count(&self) -> UBoundU8<9> {
        UBoundU8::new((self.inner & Self::ALL_MASK).count_ones() as u8)
    }

    /// Set the given value to be impossible
    pub fn set_impossible(&mut self, value: UBoundU8<9>) {
        if value == 0 {
            return;
        }
        self.inner &= !(1 << (*value - 1));
    }
}

#[derive(Clone)]
pub struct Grid {
    cells: [UBoundU8<9>; 81],
    possibilities: [CellPossibilities; 81],
}

impl Grid {
    pub fn new(cells: [UBoundU8<9>; 81]) -> Self {
        let mut possibilities = [CellPossibilities::new(); 81];

        // For each cell
        for cell_index in 0..81 {
            // Scan through the cell's neighbors
            for neighbor_index in NEIGHBOR_INDICES[cell_index] {
                if neighbor_index == cell_index {
                    continue;
                }
                let neighbor_value = &cells[neighbor_index];
                // Is the neighbor set?
                if *neighbor_value != 0 {
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

    // TODO: Do we want to make cell_value BoundedU8<1, 9> instead?
    pub fn set_cell(&mut self, cell_index: UBoundU8<80>, cell_value: UBoundU8<9>) {
        let cell_possibilities = &mut self.possibilities[*cell_index as usize];
        if !cell_possibilities.is_possible(cell_value) {
            panic!(
                "tried to assign an impossible value ({}) to cell {}",
                *cell_value, *cell_index
            );
        }

        // Assign the value to the cell
        self.cells[*cell_index as usize] = cell_value;

        // For each neighbor index
        for neighbor_index in NEIGHBOR_INDICES[*cell_index as usize] {
            if neighbor_index == *cell_index as usize {
                continue;
            }
            // Call .set_impossible(cell_value) on the neighbor's possibilities
            self.possibilities[neighbor_index].set_impossible(cell_value);
        }
    }


    /// Do all groups (rows, columns, boxes) have no repeated cell values?
    pub fn is_valid(&self) -> bool {
        // Return false if any cells in the grid don't have any legal possibilities
        for i in 0..81 {
            if !self.possibilities[i].has_possibilities() {
                return false;
            }
        }

        true
    }

    /// Is puzzle valid? Do all cells have a value?
    pub fn is_solved(&self) -> bool {
        let mut all_cells_are_set = true;
        for cell in self.cells {
            if cell == 0 {
                all_cells_are_set = false;
                break;
            }
        }

        all_cells_are_set && self.is_valid()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..=8 {
            // Print the horizontal separators between blocks
            if row == 3 || row == 6 {
                writeln!(f, "---------------------")?;
            }

            for col in 0..9 {
                // Print the vertical separators between blocks
                if col == 3 || col == 6 {
                    write!(f, "| ")?;
                }

                write!(f, "{} ", *self.cells[row * 9 + col])?
            }

            writeln!(f)?
        }
        Ok(())
    }
}
