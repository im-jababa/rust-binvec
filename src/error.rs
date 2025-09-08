/// Index out of bounds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexOutOfBounds;


impl core::fmt::Display for IndexOutOfBounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "index out of bounds")
    }
}
