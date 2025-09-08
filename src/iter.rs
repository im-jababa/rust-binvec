use crate::Binvec;


/// An iterator over a `Binvec` that yields each bit in sequence.
///
/// The iterator maintains an internal index and returns bits one by one
/// until all bits in the `Binvec` have been iterated over.
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BinvecIter<'a, const L: usize, const N: usize> {
    binvec: &'a Binvec<L, N>,
    index: usize,
}


impl<'a, const L: usize, const N: usize> BinvecIter<'a, L, N> {
    /// Creates a new `BinvecIter` for the given `Binvec`.
    ///
    /// ---
    /// # Parameters
    /// - `binvec`: A reference to the `Binvec` to iterate over.
    ///
    /// ---
    /// # Returns
    /// A new `BinvecIter` instance starting at the first bit.
    ///
    /// ---
    /// # Example
    /// ```
    /// use binvec::*;
    /// let binvec = binvec!(12, true);
    /// let mut iter = binvec.iter();
    /// assert_eq!(iter.next(), Some(true));
    /// ```
    /// 
    pub const fn new(binvec: &'a Binvec<L, N>) -> Self {
        Self { binvec, index: 0 }
    }
}


// impl Iterator
impl<'a, const L: usize, const N: usize> Iterator for BinvecIter<'a, L, N> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < L {
            // SAFETY: index is guaranteed to be less than L
            let bit: bool = unsafe { self.binvec.get_unchecked(self.index) };
            self.index += 1;
            Some(bit)
        } else {
            None
        }
    }
}
