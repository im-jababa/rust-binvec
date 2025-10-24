/// Index out of bounds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexOutOfBounds;


impl core::fmt::Display for IndexOutOfBounds {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "index out of bounds")
    }
}


impl core::error::Error for IndexOutOfBounds {}
