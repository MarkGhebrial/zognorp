use std::unreachable;

/// ROW_INDICES[i] is the set of cell indexes for the i'th row of the grid
pub const ROW_INDICES: [[usize; 9]; 9] = {
    let mut indices = [[0; 9]; 9];
    let mut i = 0;
    while i < indices.len() {
        indices[i] = row_indices(i);
        i += 1;
    }
    indices
};

/// ROW_INDICES[i] is the set of cell indexes for the i'th column of the grid
pub const COL_INDICES: [[usize; 9]; 9] = {
    let mut indices = [[0; 9]; 9];
    let mut i = 0;
    while i < indices.len() {
        indices[i] = column_indices(i);
        i += 1;
    }
    indices
};

/// BLOCK_INDICES[i] is the set of cell indexes for the i'th block of the grid
pub const BLOCK_INDICES: [[usize; 9]; 9] = {
    let mut indices = [[0; 9]; 9];
    let mut i = 0;
    while i < indices.len() {
        indices[i] = block_indices(i);
        i += 1;
    }
    indices
};

/// NEIGHBOR_INDICES[i] is the set of all cell indexes in the i'th cell's row, column, and block
pub const NEIGHBOR_INDICES: [[usize; 21]; 81] = {
    let mut indices = [[0; 21]; 81];
    let mut i = 0;
    while i < indices.len() {
        indices[i] = neighbor_indices(i);
        i += 1;
    }
    indices
};

// Currently there doesn't seem to be a way to make this const function be generic over the array item
// type
const fn array_contains(array: &[usize], value: &usize) -> bool {
    let mut i = 0;
    while i < array.len() {
        if array[i] == *value {
            return true;
        }
        i += 1;
    }
    false
}

/// Returns the indexes of the set of neighbors of a cell. The neighbors of a cell are the union of the
/// sets of the cell's block, row, and column.
const fn neighbor_indices(cell_index: usize) -> [usize; 21] {
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
    let mut indices: [usize; 21] = [usize::MAX; 21];

    // The indices of the other cells in the current cell's row, column, and block groups
    // We want to return the union of these three sets
    let row_indexes = row_indices(row_index);
    let col_indexes = column_indices(column_index);
    let block_indexes = block_indices(block_index);

    let mut current_index = 0;

    let mut i = 0;
    while i < row_indexes.len() {
        indices[current_index] = row_indexes[i];
        current_index += 1;
        i += 1;
    }

    let mut i = 0;
    while i < col_indexes.len() {
        if !array_contains(&indices, &col_indexes[i]) {
            indices[current_index] = col_indexes[i];
            current_index += 1;
        }
        i += 1;
    }

    let mut i = 0;
    while i < block_indexes.len() {
        if !array_contains(&indices, &block_indexes[i]) {
            indices[current_index] = block_indexes[i];
            current_index += 1;
        }
        i += 1;
    }

    indices
}

/// Given the index of a row, return a list of the indexes of the cells in that row
const fn row_indices(index: usize) -> [usize; 9] {
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

const fn column_indices(index: usize) -> [usize; 9] {
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
const fn block_indices(index: usize) -> [usize; 9] {
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

    let mut i = 0;
    while i < 9 {
        indices[i] = start_index + 9 * (i / 3) + (i % 3);
        i += 1;
    }

    indices
}

#[test]
fn test_block_indices() {
    assert_eq!(block_indices(0), [0, 1, 2, 9, 10, 11, 18, 19, 20]);
    assert_eq!(block_indices(1), [3, 4, 5, 12, 13, 14, 21, 22, 23]);
    assert_eq!(block_indices(2), [6, 7, 8, 15, 16, 17, 24, 25, 26]);
    assert_eq!(block_indices(3), [27, 28, 29, 36, 37, 38, 45, 46, 47]);
    assert_eq!(block_indices(4), [30, 31, 32, 39, 40, 41, 48, 49, 50]);
    assert_eq!(block_indices(5), [33, 34, 35, 42, 43, 44, 51, 52, 53]);
    assert_eq!(block_indices(6), [54, 55, 56, 63, 64, 65, 72, 73, 74]);
    assert_eq!(block_indices(7), [57, 58, 59, 66, 67, 68, 75, 76, 77]);
    assert_eq!(block_indices(8), [60, 61, 62, 69, 70, 71, 78, 79, 80]);
}
