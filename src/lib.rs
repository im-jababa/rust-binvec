/// A fixed-length array type that stores `0` or `1`.
/// It uses up to 8 times less memory compared to a [`bool`] array.
/// 
/// ---
/// # Note
/// The [`bool`] type uses 1 byte (8 bits). Therefore, storing 100 [`bool`]s requires 100 bytes (800 bits).
/// In general situations, this is not a big problem,
/// but when the array length exceeds thousands or in embedded environments with limited memory capacity,
/// this memory usage cannot be ignored.
/// 
/// To use minimal memory, calculate the minimum byte array size `N` needed to store `L` bits. The formula is as follows:
/// ```text
/// N = (L + 7) / 8
/// ```
/// 
/// For example, to store 10 bits, 2 bytes are used. In this case, the remaining 6 bits are unused.
/// ```text
/// |----- byte0 -----|----- byte1 -----|
/// | 0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0 |
///   ^^^^^^^^^^^^^^^^^^^^^ ^^^^^^^^^^^
///       10 bits used       Not used
/// ```
/// 
/// ---
/// # Generics
/// - `L`: The number of bits to store.
/// - `N`: The minimum byte array length required to store `L`.
/// 
/// ---
/// # Examples
/// ```
/// use binvec::*;
/// 
/// // A binvec storing 12 bits initialized to 0
/// let mut binvec = binvec!(12, false);
/// 
/// ```
/// 
pub struct Binvec<const L: usize, const N: usize> {
    inner: [u8; N],
}


// impl
impl<const L: usize, const N: usize> Binvec<L, N> {
    /// Creates a new [`Binvec`].
    /// 
    /// **It is not recommended to use this function. Please use [`binvec!`] macro instead.**
    /// 
    /// ---
    /// # Arguments
    /// - `initial_value`: The initial value to initialize the array.
    /// 
    /// ---
    /// # Returns
    /// An `L` length [`Binvec`] initialized with `initial_value`.
    /// 
    /// ---
    /// # Examples
    /// ```
    /// use binvec::Binvec;
    /// 
    /// let binvec = Binvec::<12, 2>::new(false);
    /// ```
    /// 
    pub const fn new(initial_value: bool) -> Self {
        let byte: u8 = if initial_value == true {
            0b1111_1111
        } else {
            0x0000_0000
        };
        Self { inner: [byte; N] }
    }

    /// Returns the bit value at the given index without performing bounds checking.
    ///
    /// ---
    /// # Safety
    /// Calling this method with an out-of-bounds index is undefined behavior.
    ///
    /// ---
    /// # Arguments
    /// - `index`: The bit index to retrieve.
    ///
    /// ---
    /// # Returns
    /// - `true` if the bit at `index` is 1.
    /// - `false` if the bit at `index` is 0.
    ///
    /// ---
    /// # Examples
    /// ```
    /// use binvec::*;
    ///
    /// let binvec = binvec!(12, true);
    /// let bit = unsafe { binvec.get_unchecked(5) };
    /// assert_eq!(bit, true);
    /// ```
    /// 
    pub const unsafe fn get_unchecked(&self, index: usize) -> bool {
        let byte_index: usize = index >> 3; // same as `index / 8`
        let bit_offset: usize = index & 0b111; // same as `index % 8`
        let byte: u8 = self.inner[byte_index];
        ((byte >> bit_offset) & 1) != 0
    }

    /// Returns the bit value at the given index with bounds checking.
    ///
    /// ---
    /// # Arguments
    /// - `index`: The bit index to retrieve.
    ///
    /// ---
    /// # Returns
    /// - `Some(true)` if the bit at `index` is 1.
    /// - `Some(false)` if the bit at `index` is 0.
    /// - `None` if `index` is out of bounds.
    ///
    /// # Examples
    /// ```
    /// use binvec::*;
    ///
    /// let binvec = binvec!(12, true);
    /// assert_eq!(binvec.get(5), Some(true));
    /// assert_eq!(binvec.get(20), None);
    /// ```
    /// 
    pub fn get(&self, index: usize) -> Option<bool> {
        if index < L {
            Some(unsafe { self.get_unchecked(index) })
        } else {
            None
        }
    }
}


/// Creates a new [`Binvec`].
/// 
/// ---
/// # Arguments
/// - `len`: Number of bits to store
/// - `initial_value`: Initial value of the array
/// 
/// ---
/// # Returns
/// An `len` length [`Binvec`] initialized with `initial_value`.
/// 
/// ---
/// # Examples
/// ```
/// use binvec::*;
/// 
/// let binvec = binvec!(12, false);
/// ```
/// 
#[macro_export]
macro_rules! binvec {
    ($len:expr, $initial_value:expr) => {{
        const L: usize = $len;
        const N: usize = (L + 7) / 8;
        Binvec::<L, N>::new($initial_value)
    }};
}
