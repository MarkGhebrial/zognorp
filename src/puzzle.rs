use std::{collections::HashSet, fmt::Display};

// Represents all the possible values that can be held in a Sudoku cell
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Cell {
    Unset,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Cell {
    pub fn is_set(&self) -> bool {
        *self != Cell::Unset
    }
}

// impl Default for Cell {
//     fn default() -> Self {
//         Self::Unset
//     }
// }

// Convert a byte to a cell
impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        assert!(value <= 9);

        use Cell::*;
        match value {
            0 => Unset,
            1 => One,
            2 => Two,
            3 => Three,
            4 => Four,
            5 => Five,
            6 => Six,
            7 => Seven,
            8 => Eight,
            9 => Nine,
            _ => unreachable!(),
        }
    }
}

// Convert a cell to a byte
impl From<Cell> for u8 {
    fn from(value: Cell) -> Self {
        value as u8
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Cell::*;
        match self {
            Unset => write!(f, " "),
            c => write!(f, "{}", *c as u8),
        }
    }
}

/// Represents a sudoku board. The cells of the board are stored in an 81 element
/// array of the Cell enum.
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Puzzle {
    cells: [Cell; 81],
}

impl Puzzle {
    pub fn new(cells: [Cell; 81]) -> Self {
        Self { cells }
    }

    // pub fn get_cell(&self, index: usize) -> Cell {
    //     self.cells[index]
    // }

    pub fn set_cell(&mut self, index: usize, cell: Cell) {
        assert!(index < 81);
        self.cells[index] = cell;
    }

    // Return an iterator over all the unset cells in the sudoku grid and their indexes.
    pub fn iter_unset_cells(&self) -> impl Iterator<Item = (usize, &Cell)> {
        self.cells.iter().enumerate().filter(|(_i, c)| !c.is_set())
    }

    /// Returns the indexes of the set of neighbors of a cell. The neighbors of a cell are the union of the
    /// sets of the cell's block, row, and column.
    pub fn neighbor_indices(cell_index: usize) -> [usize; 21] {
        let mut set_of_indexes: HashSet<usize> = HashSet::new();

        let row_index = cell_index / 9;
        let column_index = cell_index % 9;
        // Given a row and column index, figure out what 3x3 block the cell belongs to
        let block_index = match (row_index, column_index) {
            // First row of blocks
            (0..=2, 0..=2) => 0,
            (0..=2, 3..=5) => 1,
            (0..=2, 6..=8) => 2,
            // Second row of blocks
            (3..=5, 0..=2) => 3,
            (3..=5, 3..=5) => 4,
            (3..=5, 6..=8) => 5,
            // Third row of blocks
            (6..=8, 0..=2) => 6,
            (6..=8, 3..=5) => 7,
            (6..=8, 6..=8) => 8,

            _ => unreachable!(),
        };

        for index in Self::row_indices(row_index)
            .iter()
            .chain(Self::column_indices(column_index).iter())
            .chain(Self::block_indices(block_index).iter())
        {
            set_of_indexes.insert(*index);
        }

        debug_assert!(set_of_indexes.len() == 21);

        let mut indexes: [usize; 21] = [0; 21];
        for (i, index) in set_of_indexes.iter().enumerate() {
            indexes[i] = *index;
        }

        indexes
    }

    /// Given the index of a row, return a list of the indexes of the cells in that row
    pub const fn row_indices(index: usize) -> [usize; 9] {
        assert!(index < 9);

        let mut indices = [0; 9];

        let start_of_row_index = 9 * index;

        let mut i = 0;
        while i < 9 {
            indices[i] = start_of_row_index + i;
            i += 1;
        }

        indices
    }

    pub const fn column_indices(index: usize) -> [usize; 9] {
        assert!(index < 9);

        let mut indices = [0; 9];

        let mut i = 0;
        while i < 9 {
            indices[i] = index + i * 9;
            i += 1;
        }

        indices
    }

    /// Given the index of a block, return a list of the indexes of the cells in
    /// that block
    pub fn block_indices(index: usize) -> [usize; 9] {
        assert!(index < 9);

        let mut indices = [0; 9];

        // The index of the first cell in the block
        let start_index: usize = match index {
            0..=2 => index * 3,
            3..=5 => 27 + ((index % 3) * 3),
            6..=8 => 54 + ((index % 3) * 3),

            // This is unreachable thanks to the assert at the start of the function
            _ => unreachable!(),
        };

        let range1 = start_index..start_index + 3;
        let range2 = start_index + 9..start_index + 9 + 3;
        let range3 = start_index + 18..start_index + 18 + 3;

        for (i, index) in range1.chain(range2).chain(range3).enumerate() {
            indices[i] = index
        }

        indices
    }

    /// Get a row of the sudoku board
    pub fn row(&self, index: usize) -> [Cell; 9] {
        debug_assert!(index < 9);

        let mut out: [Cell; 9] = [Cell::Unset; 9];
        for (i, cell_index) in Self::row_indices(index).iter().enumerate() {
            out[i] = self.cells[*cell_index];
        }
        out
    }

    /// Get a column of the sudoku board
    pub fn column(&self, index: usize) -> [Cell; 9] {
        debug_assert!(index < 9);

        let mut out: [Cell; 9] = [Cell::Unset; 9];

        for (i, index) in Self::column_indices(index).iter().enumerate() {
            out[i] = self.cells[*index];
        }

        out
    }

    /// Get the cells in one of the puzzle's 3x3 blocks
    pub fn block(&self, index: usize) -> [Cell; 9] {
        assert!(index < 9);

        let cell_indexes = Self::block_indices(index);

        let mut out: [Cell; 9] = [Cell::Unset; 9];
        for (i, cell_index) in cell_indexes.iter().enumerate() {
            out[i] = self.cells[*cell_index];
        }

        out
    }

    pub fn possibilities(&self, cell_index: usize) -> HashSet<Cell> {
        debug_assert!(cell_index < 81);

        let row_index = cell_index / 9;
        let column_index = cell_index % 9;
        // Given a row and column index, figure out what 3x3 block the cell belongs to
        let block_index = match (row_index, column_index) {
            // First row of blocks
            (0..=2, 0..=2) => 0,
            (0..=2, 3..=5) => 1,
            (0..=2, 6..=8) => 2,
            // Second row of blocks
            (3..=5, 0..=2) => 3,
            (3..=5, 3..=5) => 4,
            (3..=5, 6..=8) => 5,
            // Third row of blocks
            (6..=8, 0..=2) => 6,
            (6..=8, 3..=5) => 7,
            (6..=8, 6..=8) => 8,

            _ => unreachable!(),
        };

        let mut set = HashSet::new();
        set.insert(Cell::One);
        set.insert(Cell::Two);
        set.insert(Cell::Three);
        set.insert(Cell::Four);
        set.insert(Cell::Five);
        set.insert(Cell::Six);
        set.insert(Cell::Seven);
        set.insert(Cell::Eight);
        set.insert(Cell::Nine);

        for cell in self
            .row(row_index)
            .iter()
            .chain(self.column(column_index).iter())
            .chain(self.block(block_index).iter())
        {
            set.remove(cell);
        }

        set
    }

    pub fn is_solved(&self) -> bool {
        if !self.is_valid() {
            return false;
        }

        for cell in self.cells {
            if !cell.is_set() {
                return false;
            }
        }

        true
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..=8 {
            // Print the horizontal separators between blocks
            if i == 3 || i == 6 {
                write!(f, "---------------------\n")?;
            }

            for j in 0..9 {
                // Print the vertical separators between blocks
                if j == 3 || j == 6 {
                    write!(f, "| ")?;
                }

                write!(f, "{} ", self.row(i)[j])?
            }

            write!(f, "\n")?
        }

        Ok(())
    }
}

pub trait Valid {
    fn is_valid(&self) -> bool;
}

impl Valid for Puzzle {
    /// Validate that the values of the cells are all valid
    fn is_valid(&self) -> bool {
        for i in 0..9 {
            if !self.row(i).is_valid() {
                println!("Row {i} is invalid");
                return false;
            }
            if !self.column(i).is_valid() {
                println!("Column {i} is invalid");
                return false;
            }
            if !self.block(i).is_valid() {
                println!("Block {i} is invalid");
                return false;
            }
        }

        true
    }
}

impl Valid for [Cell; 9] {
    fn is_valid(&self) -> bool {
        let mut set: HashSet<Cell> = HashSet::new();

        for cell in self.iter().filter(|c| c.is_set()) {
            if set.contains(cell) {
                return false;
            }
            set.insert(*cell);
        }

        true
    }
}

#[test]
fn test_group_is_valid() {
    let group: [Cell; 9] = [
        Cell::One,
        Cell::Two,
        Cell::Three,
        Cell::Four,
        Cell::Five,
        Cell::Six,
        Cell::Seven,
        Cell::Eight,
        Cell::Nine,
    ];

    assert!(group.is_valid());

    let group: [Cell; 9] = [
        Cell::One,
        Cell::Two,
        Cell::Unset,
        Cell::Four,
        Cell::Unset,
        Cell::Six,
        Cell::Seven,
        Cell::Eight,
        Cell::Unset,
    ];
    assert!(group.is_valid());

    let group: [Cell; 9] = [
        Cell::Eight,
        Cell::Two,
        Cell::Unset,
        Cell::Nine,
        Cell::Six,
        Cell::Unset,
        Cell::Unset,
        Cell::Seven,
        Cell::Four,
    ];
    assert!(group.is_valid());

    let group: [Cell; 9] = [
        Cell::Eight,
        Cell::Two,
        Cell::Seven,
        Cell::Nine,
        Cell::Six,
        Cell::Unset,
        Cell::Unset,
        Cell::Seven,
        Cell::Four,
    ];
    assert!(!group.is_valid());
}

#[test]
fn test_puzzle_is_valid() {
    #[rustfmt::skip]
    let grid: [Cell; 81] =
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

    use Cell::*;
    assert_eq!(
        puzzle.row(0),
        [Five, Three, Unset, Unset, Seven, Unset, Unset, Unset, Unset]
    );

    assert_eq!(
        puzzle.block(4),
        [Unset, Six, Unset, Eight, Unset, Three, Unset, Two, Unset,]
    );

    assert_eq!(
        puzzle.column(8),
        [Unset, Unset, Unset, Three, One, Six, Unset, Five, Nine,]
    );

    assert_eq!(
        puzzle.row(5),
        [Seven, Unset, Unset, Unset, Two, Unset, Unset, Unset, Six]
    );

    assert_eq!(
        puzzle.column(5),
        [Unset, Five, Unset, Unset, Three, Unset, Unset, Nine, Unset]
    );

    let mut set: HashSet<Cell> = HashSet::new();
    set.insert(One);
    set.insert(Four);
    assert_eq!(puzzle.possibilities(50), set,);

    assert!(puzzle.is_valid());
}
