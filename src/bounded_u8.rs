use std::ops::Deref;

/// A u8 that's guaranteed by construction to be within the upper and lower bounds. Both bounds are
/// inclusive!
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoundedU8<const LOWER: u8, const UPPER: u8> {
    inner: u8,
}

impl<const LOWER: u8, const UPPER: u8> BoundedU8<LOWER, UPPER> {
    /// Attempt to create a new BoundedU8, panicking if the provided value is out of range
    pub fn new(value: u8) -> BoundedU8<LOWER, UPPER> {
        if value < LOWER || value > UPPER {
            panic!(
                "tried to construct a BoundedU8<{}, {}> with a value of {}",
                LOWER, UPPER, value
            );
        }
        Self { inner: value }
    }
}

impl<const LOWER: u8, const UPPER: u8> PartialEq<u8> for BoundedU8<LOWER, UPPER> {
    fn eq(&self, other: &u8) -> bool {
        self.inner == *other
    }
}

impl<const LOWER: u8, const UPPER: u8> Deref for BoundedU8<LOWER, UPPER> {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// Bounded U8 with only an inclusive upper bound
pub type UBoundU8<const UPPER: u8> = BoundedU8<0, UPPER>;
