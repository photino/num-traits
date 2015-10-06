
use std::{f32, f64};
use std::num::FpCategory;

use ::Signed;

/// Float numbers.
pub trait Float: Signed
{
    /// Returns the `NaN` value.
    fn nan() -> Self;

    /// Returns the infinite value.
    fn infinity() -> Self;

    /// Returns the negative infinite value.
    fn neg_infinity() -> Self;

    /// Returns `-0.0`.
    fn neg_zero() -> Self;

    /// Returns the smallest positive, normalized value that this type can represent.
    fn min_positive_value() -> Self;

    /// Returns `true` if this value is `NaN` and false otherwise.
    fn is_nan(self) -> bool;

    /// Returns `true` if this value is positive infinity or negative infinity and false otherwise.
    fn is_infinite(self) -> bool;

    /// Returns `true` if this number is neither infinite nor `NaN`.
    fn is_finite(self) -> bool;

    /// Returns `true` if the number is neither zero, infinite, [subnormal][subnormal], or `NaN`.
    ///
    /// [subnormal]: http://en.wikipedia.org/wiki/Denormal_number
    fn is_normal(self) -> bool;

    /// Returns the floating point category of the number. If only one property is going to
    /// be tested, it is generally faster to use the specific predicate instead.
    fn classify(self) -> FpCategory;

    /// Returns the largest integer less than or equal to a number.
    fn floor(self) -> Self;

    /// Returns the smallest integer greater than or equal to a number.
    fn ceil(self) -> Self;

    /// Returns the nearest integer to a number. Round half-way cases away from `0.0`.
    fn round(self) -> Self;

    /// Return the integer part of a number.
    fn trunc(self) -> Self;

    /// Returns the fractional part of a number.
    fn fract(self) -> Self;

    /// Returns `true` if `self` is positive, including `+0.0` and `Float::infinity()`.
    fn is_sign_positive(self) -> bool;

    /// Returns `true` if `self` is negative, including `-0.0` and `Float::neg_infinity()`.
    fn is_sign_negative(self) -> bool;

    /// Fused multiply-add. Computes `(self * a) + b` with only one rounding error.
    /// This produces a more accurate result with better performance than
    /// a separate multiplication operation followed by an add.
    fn mul_add(self, a: Self, b: Self) -> Self;

    /// Take the reciprocal (inverse) of a number, `1/x`.
    fn recip(self) -> Self;

    /// Raise a number to an integer power.
    fn powi(self, n: i32) -> Self;

    /// Raise a number to a floating point power.
    fn powf(self, n: Self) -> Self;

    /// Take the square root of a number. Returns NaN if `self` is a negative number.
    fn sqrt(self) -> Self;

    /// Returns `e^(self)`, (the exponential function).
    fn exp(self) -> Self;

    /// Returns `2^(self)`.
    fn exp2(self) -> Self;

    /// Returns the natural logarithm of the number.
    fn ln(self) -> Self;

    /// Returns the logarithm of the number with respect to an arbitrary base.
    fn log(self, base: Self) -> Self;

    /// Returns the base 2 logarithm of the number.
    fn log2(self) -> Self;

    /// Returns the base 10 logarithm of the number.
    fn log10(self) -> Self;

    /// Returns the maximum of the two numbers.
    fn max(self, other: Self) -> Self;

    /// Returns the minimum of the two numbers.
    fn min(self, other: Self) -> Self;

    /// The positive difference of two numbers.
    ///
    /// * If `self <= other`: `0.0`
    /// * Else: `self - other`
    fn abs_sub(self, other: Self) -> Self;

    /// Take the cubic root of a number.
    fn cbrt(self) -> Self;

    /// Calculate the length of the hypotenuse of a right-angle triangle given legs
    /// of length `x` and `y`.
    fn hypot(self, other: Self) -> Self;

    /// Computes the sine of a number (in radians).
    fn sin(self) -> Self;

    /// Computes the cosine of a number (in radians).
    fn cos(self) -> Self;

    /// Computes the tangent of a number (in radians).
    fn tan(self) -> Self;

    /// Computes the arcsine of a number. Return value is in radians in the range
    /// [-pi/2, pi/2] or NaN if the number is outside the range [-1, 1].
    fn asin(self) -> Self;

    /// Computes the arccosine of a number. Return value is in radians in the range
    /// [0, pi] or NaN if the number is outside the range [-1, 1].
    fn acos(self) -> Self;

    /// Computes the arctangent of a number. Return value is in radians in the range
    /// [-pi/2, pi/2].
    fn atan(self) -> Self;

    /// Computes the four quadrant arctangent of `self` (`y`) and `other` (`x`).
    ///
    /// * `x = 0`, `y = 0`: `0`
    /// * `x >= 0`: `arctan(y/x)` -> `[-pi/2, pi/2]`
    /// * `y >= 0`: `arctan(y/x) + pi` -> `(pi/2, pi]`
    /// * `y < 0`: `arctan(y/x) - pi` -> `(-pi, -pi/2)`
    fn atan2(self, other: Self) -> Self;

    /// Simultaneously computes the sine and cosine of the number, `x`.
    /// Returns `(sin(x), cos(x))`.
    fn sin_cos(self) -> (Self, Self);

    /// Returns `e^(self) - 1` in a way that is accurate even if the number is close to zero.
    fn exp_m1(self) -> Self;

    /// Returns `ln(1+n)` (natural logarithm) more accurately than if
    /// the operations were performed separately.
    fn ln_1p(self) -> Self;

    /// Hyperbolic sine function.
    fn sinh(self) -> Self;

    /// Hyperbolic cosine function.
    fn cosh(self) -> Self;

    /// Hyperbolic tangent function.
    fn tanh(self) -> Self;

    /// Inverse hyperbolic sine function.
    fn asinh(self) -> Self;

    /// Inverse hyperbolic cosine function.
    fn acosh(self) -> Self;

    /// Inverse hyperbolic tangent function.
    fn atanh(self) -> Self;
}

macro_rules! impl_float {
    ($t:ident) => {
        impl Float for $t {
            fn nan() -> Self {
                $t::NAN
            }

            fn infinity() -> Self {
                $t::INFINITY
            }

            fn neg_infinity() -> Self {
                $t::NEG_INFINITY
            }

            fn neg_zero() -> Self {
                -0.0
            }

            fn min_positive_value() -> Self {
                $t::MIN_POSITIVE
            }

            fn is_nan(self) -> bool {
                <$t>::is_nan(self)
            }

            fn is_infinite(self) -> bool {
                <$t>::is_infinite(self)
            }

            fn is_finite(self) -> bool {
                <$t>::is_finite(self)
            }

            fn is_normal(self) -> bool {
                <$t>::is_normal(self)
            }

            fn classify(self) -> FpCategory {
                <$t>::classify(self)
            }

            fn floor(self) -> Self {
                <$t>::floor(self)
            }

            fn ceil(self) -> Self {
                <$t>::ceil(self)
            }

            fn round(self) -> Self {
                <$t>::round(self)
            }

            fn trunc(self) -> Self {
                <$t>::trunc(self)
            }

            fn fract(self) -> Self {
                <$t>::fract(self)
            }

            fn is_sign_positive(self) -> bool {
                <$t>::is_sign_positive(self)
            }

            fn is_sign_negative(self) -> bool {
                <$t>::is_sign_negative(self)
            }

            fn mul_add(self, a: Self, b: Self) -> Self {
                <$t>::mul_add(self, a, b)
            }

            fn recip(self) -> Self {
                <$t>::recip(self)
            }

            fn powi(self, n: i32) -> Self {
                <$t>::powi(self, n)
            }

            fn powf(self, n: Self) -> Self {
                <$t>::powf(self, n)
            }

            fn sqrt(self) -> Self {
                <$t>::sqrt(self)
            }

            fn exp(self) -> Self {
                <$t>::exp(self)
            }

            fn exp2(self) -> Self {
                <$t>::exp2(self)
            }

            fn ln(self) -> Self {
                <$t>::ln(self)
            }

            fn log(self, base: Self) -> Self {
                <$t>::log(self, base)
            }

            fn log2(self) -> Self {
                <$t>::log2(self)
            }

            fn log10(self) -> Self {
                <$t>::log10(self)
            }

            fn max(self, other: Self) -> Self {
                <$t>::max(self, other)
            }

            fn min(self, other: Self) -> Self {
                <$t>::min(self, other)
            }

            fn abs_sub(self, other: Self) -> Self {
                <$t>::abs_sub(self, other)
            }

            fn cbrt(self) -> Self {
                <$t>::cbrt(self)
            }

            fn hypot(self, other: Self) -> Self {
                <$t>::hypot(self, other)
            }

            fn sin(self) -> Self {
                <$t>::sin(self)
            }

            fn cos(self) -> Self {
                <$t>::cos(self)
            }

            fn tan(self) -> Self {
                <$t>::tan(self)
            }

            fn asin(self) -> Self {
                <$t>::asin(self)
            }

            fn acos(self) -> Self {
                <$t>::acos(self)
            }

            fn atan(self) -> Self {
                <$t>::atan(self)
            }

            fn atan2(self, other: Self) -> Self {
                <$t>::atan2(self, other)
            }

            fn sin_cos(self) -> (Self, Self) {
                <$t>::sin_cos(self)
            }

            fn exp_m1(self) -> Self {
                <$t>::exp_m1(self)
            }

            fn ln_1p(self) -> Self {
                <$t>::ln_1p(self)
            }

            fn sinh(self) -> Self {
                <$t>::sinh(self)
            }

            fn cosh(self) -> Self {
                <$t>::cosh(self)
            }

            fn tanh(self) -> Self {
                <$t>::tanh(self)
            }

            fn asinh(self) -> Self {
                <$t>::asinh(self)
            }

            fn acosh(self) -> Self {
                <$t>::acosh(self)
            }

            fn atanh(self) -> Self {
                <$t>::atanh(self)
            }
        }
    }
}

impl_float!(f32);
impl_float!(f64);
