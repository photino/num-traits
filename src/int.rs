
use std::ops::{Add, Sub, Mul, Div, Rem, Not, BitAnd, BitOr, BitXor, Shl, Shr};
use std::num::ParseIntError;

use ::{Zero, One};

/// Signed and unsigned integers.
pub trait Int: Copy + Clone + PartialOrd + PartialEq +
               Zero + One + Add<Output = Self> + Sub<Output = Self> +
               Mul<Output = Self> + Div<Output = Self> + Rem<Output = Self> +
               Not<Output = Self> + BitAnd<Output = Self> + BitOr<Output = Self> +
               BitXor<Output = Self> + Shl<usize, Output = Self> + Shr<usize, Output = Self>
{
    /// Returns the smallest value that can be represented by this numeric type.
    fn min_value() -> Self;

    /// Returns the largest value that can be represented by this numeric type.
    fn max_value() -> Self;

    /// Converts a string slice in a given base to an integer.
    fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError>;

    /// Returns the number of ones in the binary representation of `self`.
    fn count_ones(self) -> u32;

    /// Returns the number of zeros in the binary representation of `self`.
    fn count_zeros(self) -> u32;

    /// Returns the number of leading zeros in the binary representation of `self`.
    fn leading_zeros(self) -> u32;

    /// Returns the number of trailing zeros in the binary representation of `self`.
    fn trailing_zeros(self) -> u32;

    /// Shifts the bits to the left by a specified amount, `n`,
    /// wrapping the truncated bits to the end of the resulting integer.
    fn rotate_left(self, n: u32) -> Self;

    /// Shifts the bits to the right by a specified amount, `n`,
    /// wrapping the truncated bits to the beginning of the resulting integer.
    fn rotate_right(self, n: u32) -> Self;

    /// Reverses the byte order of the integer.
    fn swap_bytes(self) -> Self;

    /// Converts an integer from big endian to the target's endianness.
    fn from_be(x: Self) -> Self;

    /// Converts an integer from little endian to the target's endianness.
    fn from_le(x: Self) -> Self;

    /// Converts `self` to big endian from the target's endianness.
    fn to_be(self) -> Self;

    /// Converts `self` to little endian from the target's endianness.
    fn to_le(self) -> Self;

    /// Checked integer addition. Computes `self + other`,
    /// returning `None` if overflow occurred.
    fn checked_add(self, other: Self) -> Option<Self>;

    /// Checked integer subtraction. Computes `self - other`,
    /// returning `None` if underflow occurred.
    fn checked_sub(self, other: Self) -> Option<Self>;

    /// Checked integer multiplication. Computes `self * other`,
    /// returning `None` if underflow or overflow occurred.
    fn checked_mul(self, other: Self) -> Option<Self>;

    /// Checked integer division. Computes `self / other`,
    /// returning `None` if `other == 0` or the operation results in underflow or overflow.
    fn checked_div(self, other: Self) -> Option<Self>;

    /// Saturating integer addition. Computes `self + other`,
    /// saturating at the numeric bounds instead of overflowing.
    fn saturating_add(self, other: Self) -> Self;

    /// Saturating integer subtraction. Computes `self - other`,
    /// saturating at the numeric bounds instead of overflowing.
    fn saturating_sub(self, other: Self) -> Self;

    /// Wrapping (modular) addition. Computes `self + other`,
    /// wrapping around at the boundary of the type.
    fn wrapping_add(self, rhs: Self) -> Self;

    /// Wrapping (modular) subtraction. Computes `self - other`,
    /// wrapping around at the boundary of the type.
    fn wrapping_sub(self, rhs: Self) -> Self;

    /// Wrapping (modular) multiplication. Computes `self * other`,
    /// wrapping around at the boundary of the type.
    fn wrapping_mul(self, rhs: Self) -> Self;

    /// Wrapping (modular) division. Computes `self / other`,
    /// wrapping around at the boundary of the type.
    fn wrapping_div(self, rhs: Self) -> Self;

    /// Wrapping (modular) remainder. Computes `self % other`,
    /// wrapping around at the boundary of the type.
    fn wrapping_rem(self, rhs: Self) -> Self;

    /// Wrapping (modular) negation. Computes ``-self`,
    /// wrapping around at the boundary of the type.
    fn wrapping_neg(self) -> Self;

    /// Panic-free bitwise shift-left; yields `self << mask(rhs)`,
    /// where `mask` removes any high-order bits of rhs that would
    /// cause the shift to exceed the bitwidth of the type.
    fn wrapping_shl(self, rhs: u32) -> Self;

    /// Panic-free bitwise shift-left; yields `self >> mask(rhs)`,
    /// where mask removes any high-order bits of rhs that would
    /// cause the shift to exceed the bitwidth of the type.
    fn wrapping_shr(self, rhs: u32) -> Self;

    /// Raises self to the power of `exp`, using exponentiation by squaring.
    fn pow(self, exp: u32) -> Self;
}

macro_rules! impl_int {
    ($($t:ty)*) => {
        $(
            impl Int for $t {
                fn min_value() -> Self {
                    <$t>::min_value()
                }

                fn max_value() -> Self {
                    <$t>::max_value()
                }

                fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> {
                    <$t>::from_str_radix(src, radix)
                }

                fn count_ones(self) -> u32 {
                    <$t>::count_ones(self)
                }

                fn count_zeros(self) -> u32 {
                    <$t>::count_zeros(self)
                }

                fn leading_zeros(self) -> u32 {
                    <$t>::leading_zeros(self)
                }

                fn trailing_zeros(self) -> u32 {
                    <$t>::trailing_zeros(self)
                }

                fn rotate_left(self, n: u32) -> Self {
                    <$t>::rotate_left(self, n)
                }

                fn rotate_right(self, n: u32) -> Self {
                    <$t>::rotate_right(self, n)
                }

                fn swap_bytes(self) -> Self {
                    <$t>::swap_bytes(self)
                }

                fn from_be(x: Self) -> Self {
                    <$t>::from_be(x)
                }

                fn from_le(x: Self) -> Self {
                    <$t>::from_le(x)
                }

                fn to_be(self) -> Self {
                    <$t>::to_be(self)
                }

                fn to_le(self) -> Self {
                    <$t>::to_le(self)
                }

                fn checked_add(self, other: Self) -> Option<Self> {
                    <$t>::checked_add(self, other)
                }

                fn checked_sub(self, other: Self) -> Option<Self> {
                    <$t>::checked_sub(self, other)
                }

                fn checked_mul(self, other: Self) -> Option<Self> {
                    <$t>::checked_mul(self, other)
                }

                fn checked_div(self, other: Self) -> Option<Self> {
                    <$t>::checked_div(self, other)
                }

                fn saturating_add(self, other: Self) -> Self {
                    <$t>::saturating_add(self, other)
                }

                fn saturating_sub(self, other: Self) -> Self {
                    <$t>::saturating_sub(self, other)
                }

                fn wrapping_add(self, rhs: Self) -> Self {
                    <$t>::wrapping_add(self, rhs)
                }

                fn wrapping_sub(self, rhs: Self) -> Self {
                    <$t>::wrapping_sub(self, rhs)
                }

                fn wrapping_mul(self, rhs: Self) -> Self {
                    <$t>::wrapping_mul(self, rhs)
                }

                fn wrapping_div(self, rhs: Self) -> Self {
                    <$t>::wrapping_div(self, rhs)
                }

                fn wrapping_rem(self, rhs: Self) -> Self {
                    <$t>::wrapping_rem(self, rhs)
                }

                fn wrapping_neg(self) -> Self {
                    <$t>::wrapping_neg(self)
                }

                fn wrapping_shl(self, rhs: u32) -> Self {
                    <$t>::wrapping_shl(self, rhs)
                }

                fn wrapping_shr(self, rhs: u32) -> Self {
                    <$t>::wrapping_shr(self, rhs)
                }

                fn pow(self, exp: u32) -> Self {
                    <$t>::pow(self, exp)
                }
            }
        )*
    }
}

impl_int!(u8 u16 u32 u64 usize i8 i16 i32 i64 isize);

/// Unsigned integers.
pub trait UnsignedInt: Int {
    /// Returns true if and only if `self == 2^k` for some `k`.
    fn is_power_of_two(self) -> bool;

    /// Returns the smallest power of two greater than or equal to `self`.
    /// Unspecified behavior on overflow.
    fn next_power_of_two(self) -> Self;

    /// Returns the smallest power of two greater than or equal to `n`.
    /// If the next power of two is greater than the type's maximum value,
    /// `None` is returned, otherwise the power of two is wrapped in `Some`.
    fn checked_next_power_of_two(self) -> Option<Self>;
}

macro_rules! impl_unsigned_int {
    ($($t:ty)*) => {
        $(
            impl UnsignedInt for $t {
                fn is_power_of_two(self) -> bool {
                    <$t>::is_power_of_two(self)
                }

                fn next_power_of_two(self) -> Self {
                    <$t>::next_power_of_two(self)
                }

                fn checked_next_power_of_two(self) -> Option<Self> {
                    <$t>::checked_next_power_of_two(self)
                }
            }
        )*
    }
}

impl_unsigned_int!(u8 u16 u32 u64 usize);
