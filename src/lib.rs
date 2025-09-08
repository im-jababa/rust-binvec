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
