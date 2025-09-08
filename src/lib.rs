pub mod error;

mod iter;
pub use iter::*;


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
/// ```
/// 
#[derive(Debug, Clone, PartialEq, Eq)]
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
    #[deprecated(note = "Use the `binvec!` macro instead.")]
    #[doc(hidden)]
    pub const fn new(initial_value: bool) -> Self {
        let mut new: Binvec<L, N> = Self { inner: [0x00; N] };
        new.fill(initial_value);
        new
    }

    /// Returns the length in bits of the [`Binvec`].
    ///
    /// ---
    /// # Returns
    /// The number of bits stored in the [`Binvec`].
    ///
    /// ---
    /// # Examples
    /// ```
    /// use binvec::*;
    ///
    /// let binvec = binvec!(12, false);
    /// assert_eq!(binvec.len(), 12);
    /// ```
    /// 
    #[inline(always)]
    pub const fn len(&self) -> usize {
        L
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
    #[inline]
    pub fn get(&self, index: usize) -> Option<bool> {
        if index < L {
            Some(unsafe { self.get_unchecked(index) })
        } else {
            None
        }
    }

    /// Sets the bit value at the given index without performing bounds checking.
    ///
    /// ---
    /// # Safety
    /// Calling this method with an out-of-bounds index is undefined behavior.
    ///
    /// ---
    /// # Arguments
    /// - `index`: The bit index to set.
    /// - `value`: The bit value to set (`true` for 1, `false` for 0).
    ///
    /// ---
    /// # Examples
    /// ```
    /// use binvec::*;
    ///
    /// let mut binvec = binvec!(12, false);
    /// unsafe { binvec.set_unchecked(3, true); }
    /// assert_eq!(binvec.get(3), Some(true));
    /// ```
    /// 
    pub unsafe fn set_unchecked(&mut self, index: usize, value: bool) {
        let byte_index: usize = index >> 3; // same as `index / 8`
        let bit_offset: usize = index & 0b111; // same as `index % 8`
        let mask: u8 = 1 << bit_offset;
        let byte: &mut u8 = &mut self.inner[byte_index];
        if value == true {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }

    /// Sets the bit value at the given index with bounds checking.
    ///
    /// ---
    /// # Arguments
    /// - `index`: The bit index to set.
    /// - `value`: The bit value to set (`true` for 1, `false` for 0).
    ///
    /// ---
    /// # Returns
    /// - `Ok(())` if the bit was successfully set.
    /// - `Err(())` if the index is out of bounds.
    ///
    /// ---
    /// # Examples
    /// ```
    /// use binvec::*;
    ///
    /// let mut binvec = binvec!(12, false);
    /// assert_eq!(binvec.set(5, true), Ok(()));
    /// assert_eq!(binvec.get(5), Some(true));
    ///
    /// assert_eq!(binvec.set(20, true), Err(error::IndexOutOfBounds));
    /// ```
    /// 
    #[inline]
    pub fn set(&mut self, index: usize, value: bool) -> Result<(), error::IndexOutOfBounds> {
        if index < L {
            unsafe { self.set_unchecked(index, value); }
            Ok(())
        } else {
            Err(error::IndexOutOfBounds)
        }
    }

    /// Fills the entire [`Binvec`] with the specified bit value.
    ///
    /// This method sets all bits in the [`Binvec`] to either `true` (1) or `false` (0).
    /// For bits beyond the length `L` in the last byte, those bits are cleared to zero.
    ///
    /// ---
    /// # Arguments
    /// - `value`: The bit value to fill the [`Binvec`] with (`true` for 1, `false` for 0).
    ///
    /// ---
    /// # Examples
    /// ```
    /// use binvec::*;
    ///
    /// let mut binvec = binvec!(12, false);
    /// binvec.fill(true);
    /// assert_eq!(binvec.is_all_one(), true);
    /// ```
    /// 
    pub const fn fill(&mut self, value: bool) {
        let byte: u8 = if value == true { 0xFF } else { 0x00 };
        let mut inner: [u8; N] = [byte; N];
        if L > 0
        && L % 8 != 0 {
            let last_bits: usize = L % 8;
            let mask: u8 = (1u8 << last_bits) - 1;
            inner[N - 1] &= mask;
        }
        self.inner = inner;
    }

    /// Counts the number of bits set to `1` in the [`Binvec`].
    ///
    /// ---
    /// # Returns
    /// The total count of bits that are set to `1`.
    ///
    /// ---
    /// # Examples
    /// ```
    /// use binvec::*;
    ///
    /// let binvec = binvec!(12, true);
    /// assert_eq!(binvec.count_ones(), 12);
    ///
    /// let mut binvec = binvec!(12, false);
    /// binvec.set(3, true).unwrap();
    /// assert_eq!(binvec.count_ones(), 1);
    /// ```
    /// 
    pub const fn count_ones(&self) -> usize {
        let mut count: usize = 0;
        let mut i: usize = 0;
        while i < N {
            count += self.inner[i].count_ones() as usize;
            i += 1;
        }
        count // unused value is always filled with 0
    }

    /// Counts the number of bits set to `0` in the [`Binvec`].
    ///
    /// ---
    /// # Returns
    /// The total count of bits that are set to `0`.
    ///
    /// ---
    /// # Examples
    /// ```
    /// use binvec::*;
    ///
    /// let binvec = binvec!(12, false);
    /// assert_eq!(binvec.count_zeros(), 12);
    ///
    /// let mut binvec = binvec!(12, true);
    /// binvec.set(3, false).unwrap();
    /// assert_eq!(binvec.count_zeros(), 1);
    /// ```
    /// 
    pub const fn count_zeros(&self) -> usize {
        let mut count: usize = 0;
        let mut i: usize = 0;
        while i < N {
            count += self.inner[i].count_zeros() as usize;
            i += 1;
        }
        count - ((N * 8) - L) // unused value is always filled with 0
    }

    /// Checks if all bits in the [`Binvec`] are set to `1`.
    ///
    /// ---
    /// # Returns
    /// `true` if all bits that are using is `1`, otherwise `false`.
    ///
    /// ---
    /// # Examples
    /// ```
    /// use binvec::*;
    ///
    /// let binvec = binvec!(12, true);
    /// assert_eq!(binvec.is_all_one(), true);
    ///
    /// let mut binvec = binvec!(12, false);
    /// assert_eq!(binvec.is_all_one(), false);
    /// binvec.fill(true);
    /// assert_eq!(binvec.is_all_one(), true);
    /// ```
    /// 
    #[inline(always)]
    pub const fn is_all_one(&self) -> bool {
        self.count_ones() == L
    }

    /// Checks if all bits in the [`Binvec`] are set to `0`.
    ///
    /// ---
    /// # Returns
    /// `true` if all bits that are using is `0`, otherwise `false`.
    ///
    /// ---
    /// # Examples
    /// ```
    /// use binvec::*;
    ///
    /// let binvec = binvec!(12, false);
    /// assert_eq!(binvec.is_all_zero(), true);
    ///
    /// let mut binvec = binvec!(12, true);
    /// assert_eq!(binvec.is_all_zero(), false);
    /// binvec.fill(false);
    /// assert_eq!(binvec.is_all_zero(), true);
    /// ```
    /// 
    #[inline(always)]
    pub const fn is_all_zero(&self) -> bool {
        self.count_zeros() == L
    }

    /// Returns an iterator over the bits of the [`Binvec`].
    ///
    /// ---
    /// # Returns
    /// A [`BinvecIter`] that yields each bit as a `bool`.
    ///
    /// ---
    /// # Examples
    /// ```
    /// use binvec::*;
    ///
    /// let binvec = binvec!(12, true);
    /// for bit in binvec.iter() {
    ///     assert_eq!(bit, true);
    /// }
    /// ```
    /// 
    #[inline(always)]
    pub fn iter(&self) -> BinvecIter<'_, L, N> {
        BinvecIter::new(self)
    }
}


// impl IntoIterator
impl<'a, const L: usize, const N: usize> IntoIterator for &'a Binvec<L, N> {
    type Item = bool;
    type IntoIter = BinvecIter<'a, L, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
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
        const N: usize = (L + 7) >> 3; // same as (L + 7) / 8
        #[allow(deprecated)]
        Binvec::<L, N>::new($initial_value)
    }};
}


// impl Display
impl<const L: usize, const N: usize> core::fmt::Display for Binvec<L, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[")?;
        let mut iter: BinvecIter<'_, L, N> = self.iter();
        if let Some(first) = iter.next() {
            write!(f, "{}", if first { "1" } else { "0" })?;
            for bit in iter {
                write!(f, ", {}", if bit { "1" } else { "0" })?;
            }
        }
        write!(f, "]")
    }
}
