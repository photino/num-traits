//! Numeric traits for generic mathematics.

use std::mem::size_of;

/// Reexports.
pub use int::{Int, UnsignedInt};
pub use float::Float;
pub use signed::Signed;

mod int;
mod float;
mod signed;

/// Types that have a `zero` value.
///
/// This trait is intended for use in conjunction with `Add`, as an identity:
/// `x + T::zero() == x`.
pub trait Zero {
    /// Returns the `zero` (usually, additive identity) for this type.
    fn zero() -> Self;
}

/// Types that have a `one` value.
///
/// This trait is intended for use in conjunction with `Mul`, as an identity:
/// `x * T::one() == x`.
pub trait One {
    /// Returns the `one` (usually, multiplicative identity) for this type.
    fn one() -> Self;
}

macro_rules! impl_zero_one_int {
    ($($t:ty)*) => {
        $(
            impl Zero for $t {
                #[inline(always)]
                fn zero() -> Self { 0 }
            }

            impl One for $t {
                #[inline(always)]
                fn one() -> Self { 1 }
            }
        )*
    }
}

impl_zero_one_int!(u8 u16 u32 u64 usize i8 i16 i32 i64 isize);

macro_rules! impl_zero_one_float {
    ($($t:ty)*) => {
        $(
            impl Zero for $t {
                #[inline(always)]
                fn zero() -> Self { 0.0 }
            }

            impl One for $t {
                #[inline(always)]
                fn one() -> Self { 1.0 }
            }
        )*
    }
}

impl_zero_one_float!(f32 f64);

/// Constructs `Self` from the other type via a conversion.
pub trait CastFrom<T>: Sized {
    /// Constructs `Self` from the type `T`.
    fn cast_from(x: T) -> Option<Self>;
}

/// Converts `Self` into the other type.
pub trait CastInto<T> {
    /// Casts `Self` into the type `T`.
    fn cast_into(self) -> Option<T>;
}

impl<S, T> CastInto<T> for S
    where T: CastFrom<S>
{
    fn cast_into(self) -> Option<T> {
        <T>::cast_from(self)
    }
}

macro_rules! impl_cast_same_type {
    ($T:ty) => {
        impl CastFrom<$T> for $T {
            fn cast_from(x: $T) -> Option<$T> {
                Some(x)
            }
        }
    }
}

macro_rules! impl_cast_int_to_int {
    ($S:ty, $T:ty) => {
        impl CastFrom<$S> for $T {
            fn cast_from(x: $S) -> Option<$T> {
                if size_of::<$S>() <= size_of::<$T>() {
                    Some(x as $T)
                } else {
                    let n = x as i64;
                    let min_value = <$T>::min_value();
                    let max_value = <$T>::max_value();
                    if min_value as i64 <= n && n <= max_value as i64 {
                        Some(x as $T)
                    } else {
                        None
                    }
                }
            }
        }
    }
}

macro_rules! impl_cast_int_to_uint {
    ($S:ty, $T:ty) => {
        impl CastFrom<$S> for $T {
            fn cast_from(x: $S) -> Option<$T> {
                let zero = <$S>::zero();
                let max_value = <$T>::max_value();
                if zero <= x && x as u64 <= max_value as u64 {
                    Some(x as $T)
                } else {
                    None
                }
            }
        }
    }
}

macro_rules! impl_cast_int_to_float {
    ($S:ty, $T:ty) => {
        impl CastFrom<$S> for $T {
            fn cast_from(x: $S) -> Option<$T> {
                Some(x as $T)
            }
        }
    }
}

macro_rules! impl_cast_uint_to_int {
    ($S:ty, $T:ty) => {
        impl CastFrom<$S> for $T {
            fn cast_from(x: $S) -> Option<$T> {
                let max_value = <$T>::max_value();
                if x as u64 <= max_value as u64 {
                    Some(x as $T)
                } else {
                    None
                }
            }

        }
    }
}

macro_rules! impl_cast_uint_to_uint {
    ($S:ty, $T:ty) => {
        impl CastFrom<$S> for $T {
            fn cast_from(x: $S) -> Option<$T> {
                if size_of::<$S>() <= size_of::<$T>() {
                    Some(x as $T)
                } else {
                    let zero = <$S>::zero();
                    let max_value = <$T>::max_value();
                    if zero <= x && x as u64 <= max_value as u64 {
                        Some(x as $T)
                    } else {
                        None
                    }
                }
            }
        }
    }
}

macro_rules! impl_cast_float_to_int {
    ($S:ty, $T:ty) => {
        impl CastFrom<$S> for $T {
            fn cast_from(x: $S) -> Option<$T> {
                let min_value = <$T>::min_value();
                let max_value = <$T>::max_value();
                if min_value as $S <= x && x <= max_value as $S {
                    Some(x as $T)
                } else {
                    None
                }
            }
        }
    }
}

macro_rules! impl_cast_float_to_uint {
    ($S:ty, $T:ty) => {
        impl CastFrom<$S> for $T {
            fn cast_from(x: $S) -> Option<$T> {
                let zero = <$S>::zero();
                let max_value = <$T>::max_value();
                if zero <= x && x <= max_value as $S {
                    Some(x as $T)
                } else {
                    None
                }
            }
        }
    }
}

macro_rules! impl_cast_float_to_float {
    ($S:ty, $T:ty) => {
        impl CastFrom<$S> for $T {
            fn cast_from(x: $S) -> Option<$T> {
                if size_of::<$S>() <= size_of::<$T>() {
                    Some(x as $T)
                } else {
                    let y = x as f64;
                    let max_value = <$S>::max_value();
                    if -max_value as f64 <= y && y <= max_value as f64 {
                        Some(x as $T)
                    } else {
                        None
                    }
                }
            }
        }
    }
}

impl_cast_same_type!(i8);
impl_cast_int_to_int!(i8, i16);
impl_cast_int_to_int!(i8, i32);
impl_cast_int_to_int!(i8, i64);
impl_cast_int_to_int!(i8, isize);
impl_cast_int_to_uint!(i8, u8);
impl_cast_int_to_uint!(i8, u16);
impl_cast_int_to_uint!(i8, u32);
impl_cast_int_to_uint!(i8, u64);
impl_cast_int_to_uint!(i8, usize);
impl_cast_int_to_float!(i8, f32);
impl_cast_int_to_float!(i8, f64);

impl_cast_int_to_int!(i16, i8);
impl_cast_same_type!(i16);
impl_cast_int_to_int!(i16, i32);
impl_cast_int_to_int!(i16, i64);
impl_cast_int_to_int!(i16, isize);
impl_cast_int_to_uint!(i16, u8);
impl_cast_int_to_uint!(i16, u16);
impl_cast_int_to_uint!(i16, u32);
impl_cast_int_to_uint!(i16, u64);
impl_cast_int_to_uint!(i16, usize);
impl_cast_int_to_float!(i16, f32);
impl_cast_int_to_float!(i16, f64);

impl_cast_int_to_int!(i32, i8);
impl_cast_int_to_int!(i32, i16);
impl_cast_same_type!(i32);
impl_cast_int_to_int!(i32, i64);
impl_cast_int_to_int!(i32, isize);
impl_cast_int_to_uint!(i32, u8);
impl_cast_int_to_uint!(i32, u16);
impl_cast_int_to_uint!(i32, u32);
impl_cast_int_to_uint!(i32, u64);
impl_cast_int_to_uint!(i32, usize);
impl_cast_int_to_float!(i32, f32);
impl_cast_int_to_float!(i32, f64);

impl_cast_int_to_int!(i64, i8);
impl_cast_int_to_int!(i64, i16);
impl_cast_int_to_int!(i64, i32);
impl_cast_same_type!(i64);
impl_cast_int_to_int!(i64, isize);
impl_cast_int_to_uint!(i64, u8);
impl_cast_int_to_uint!(i64, u16);
impl_cast_int_to_uint!(i64, u32);
impl_cast_int_to_uint!(i64, u64);
impl_cast_int_to_uint!(i64, usize);
impl_cast_int_to_float!(i64, f32);
impl_cast_int_to_float!(i64, f64);

impl_cast_int_to_int!(isize, i8);
impl_cast_int_to_int!(isize, i16);
impl_cast_int_to_int!(isize, i32);
impl_cast_int_to_int!(isize, i64);
impl_cast_same_type!(isize);
impl_cast_int_to_uint!(isize, u8);
impl_cast_int_to_uint!(isize, u16);
impl_cast_int_to_uint!(isize, u32);
impl_cast_int_to_uint!(isize, u64);
impl_cast_int_to_uint!(isize, usize);
impl_cast_int_to_float!(isize, f32);
impl_cast_int_to_float!(isize, f64);

impl_cast_uint_to_uint!(u8, i8);
impl_cast_uint_to_int!(u8, i16);
impl_cast_uint_to_int!(u8, i32);
impl_cast_uint_to_int!(u8, i64);
impl_cast_uint_to_int!(u8, isize);
impl_cast_same_type!(u8);
impl_cast_uint_to_uint!(u8, u16);
impl_cast_uint_to_uint!(u8, u32);
impl_cast_uint_to_uint!(u8, u64);
impl_cast_uint_to_uint!(u8, usize);
impl_cast_int_to_float!(u8, f32);
impl_cast_int_to_float!(u8, f64);

impl_cast_uint_to_uint!(u16, i8);
impl_cast_uint_to_int!(u16, i16);
impl_cast_uint_to_int!(u16, i32);
impl_cast_uint_to_int!(u16, i64);
impl_cast_uint_to_int!(u16, isize);
impl_cast_uint_to_uint!(u16, u8);
impl_cast_same_type!(u16);
impl_cast_uint_to_uint!(u16, u32);
impl_cast_uint_to_uint!(u16, u64);
impl_cast_uint_to_uint!(u16, usize);
impl_cast_int_to_float!(u16, f32);
impl_cast_int_to_float!(u16, f64);

impl_cast_uint_to_uint!(u32, i8);
impl_cast_uint_to_int!(u32, i16);
impl_cast_uint_to_int!(u32, i32);
impl_cast_uint_to_int!(u32, i64);
impl_cast_uint_to_int!(u32, isize);
impl_cast_uint_to_uint!(u32, u8);
impl_cast_uint_to_uint!(u32, u16);
impl_cast_same_type!(u32);
impl_cast_uint_to_uint!(u32, u64);
impl_cast_uint_to_uint!(u32, usize);
impl_cast_int_to_float!(u32, f32);
impl_cast_int_to_float!(u32, f64);

impl_cast_uint_to_uint!(u64, i8);
impl_cast_uint_to_int!(u64, i16);
impl_cast_uint_to_int!(u64, i32);
impl_cast_uint_to_int!(u64, i64);
impl_cast_uint_to_int!(u64, isize);
impl_cast_uint_to_uint!(u64, u8);
impl_cast_uint_to_uint!(u64, u16);
impl_cast_uint_to_uint!(u64, u32);
impl_cast_same_type!(u64);
impl_cast_uint_to_uint!(u64, usize);
impl_cast_int_to_float!(u64, f32);
impl_cast_int_to_float!(u64, f64);

impl_cast_uint_to_uint!(usize, i8);
impl_cast_uint_to_int!(usize, i16);
impl_cast_uint_to_int!(usize, i32);
impl_cast_uint_to_int!(usize, i64);
impl_cast_uint_to_int!(usize, isize);
impl_cast_uint_to_uint!(usize, u8);
impl_cast_uint_to_uint!(usize, u16);
impl_cast_uint_to_uint!(usize, u32);
impl_cast_uint_to_uint!(usize, u64);
impl_cast_same_type!(usize);
impl_cast_int_to_float!(usize, f32);
impl_cast_int_to_float!(usize, f64);

impl_cast_float_to_int!(f32, i8);
impl_cast_float_to_int!(f32, i16);
impl_cast_float_to_int!(f32, i32);
impl_cast_float_to_int!(f32, i64);
impl_cast_float_to_int!(f32, isize);
impl_cast_float_to_uint!(f32, u8);
impl_cast_float_to_uint!(f32, u16);
impl_cast_float_to_uint!(f32, u32);
impl_cast_float_to_uint!(f32, u64);
impl_cast_float_to_uint!(f32, usize);
impl_cast_same_type!(f32);
impl_cast_float_to_float!(f32, f64);

impl_cast_float_to_int!(f64, i8);
impl_cast_float_to_int!(f64, i16);
impl_cast_float_to_int!(f64, i32);
impl_cast_float_to_int!(f64, i64);
impl_cast_float_to_int!(f64, isize);
impl_cast_float_to_uint!(f64, u8);
impl_cast_float_to_uint!(f64, u16);
impl_cast_float_to_uint!(f64, u32);
impl_cast_float_to_uint!(f64, u64);
impl_cast_float_to_uint!(f64, usize);
impl_cast_float_to_float!(f64, f32);
impl_cast_same_type!(f64);

#[test]
fn test_cast() {
    let a = 32i32;
    let b = f32::cast_from(a).unwrap();
    let c: Option<i32> = 1.0e+123f64.cast_into();
    assert_eq!(b, 32.0f32);
    assert_eq!(c, None);
}
