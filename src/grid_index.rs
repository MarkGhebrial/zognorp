use std::ops::Deref;

use crate::bounded_u8::UBoundU8;

pub struct GridIndex {
    /// A u8 between 0 and 80, inclusive. Perfect for an 81 cell grid
    value: UBoundU8<{ 9 * 9 - 1 }>,
}

impl Deref for GridIndex {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GridIndex {
    pub fn from_row_col(row: UBoundU8<8>, col: UBoundU8<8>) -> Self {
        Self {
            value: UBoundU8::new((*row * 9) + *col),
        }
    }

    /// Get the row number of the index
    pub fn get_row(&self) -> UBoundU8<8> {
        UBoundU8::new(*self.value / 9)
    }

    /// Get the column number of the index
    pub fn get_col(&self) -> UBoundU8<8> {
        UBoundU8::new(*self.value % 9)
    }

    // pub fn get_row_neighbor_indices(&self) -> [GridIndex; 8] {
    //
    // }
}
