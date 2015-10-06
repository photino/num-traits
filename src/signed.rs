
use std::{f32, f64};
use std::ops::Neg;

/// Signed numbers.
pub trait Signed: Sized + Neg<Output = Self> {
    /// Computes the absolute value.
    ///
    /// For `f32` and `f64`, `NaN` will be returned if the number is `NaN`.
    ///
    /// For signed integers, `::MIN` will be returned if the number is `::MIN`.
    fn abs(&self) -> Self;

    /// Returns the sign of the number.
    ///
    /// For `f32` and `f64`:
    ///
    /// * `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// * `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// * `NaN` if the number is `NaN`
    ///
    /// For signed integers:
    ///
    /// * `0` if the number is zero
    /// * `1` if the number is positive
    /// * `-1` if the number is negative
    fn signum(&self) -> Self;

    /// Returns true if the number is positive and false if the number is zero or negative.
    fn is_positive(&self) -> bool;

    /// Returns true if the number is negative and false if the number is zero or positive.
    fn is_negative(&self) -> bool;
}

macro_rules! impl_signed_int {
    ($($t:ty)*) => {
        $(
            impl Signed for $t {
                #[inline]
                fn abs(&self) -> $t {
                    <$t>::abs(*self)
                }

                #[inline]
                fn signum(&self) -> $t {
                    <$t>::signum(*self)
                }

                #[inline]
                fn is_positive(&self) -> bool { *self > 0 }

                #[inline]
                fn is_negative(&self) -> bool { *self < 0 }
            }
        )*
    }
}

impl_signed_int!(i8 i16 i32 i64 isize);

macro_rules! impl_signed_float {
    ($t:ty, $inf:expr, $neg_inf:expr) => {
        impl Signed for $t {
            #[inline]
            fn abs(&self) -> $t {
                <$t>::abs(*self)
            }

            #[inline]
            fn signum(&self) -> $t {
                <$t>::signum(*self)
            }

            #[inline]
            fn is_positive(&self) -> bool { *self > 0.0 || (1.0 / *self) == $inf }

            #[inline]
            fn is_negative(&self) -> bool { *self < 0.0 || (1.0 / *self) == $neg_inf }
        }
    }
}

impl_signed_float!(f32, f32::INFINITY, f32::NEG_INFINITY);
impl_signed_float!(f64, f64::INFINITY, f64::NEG_INFINITY);
