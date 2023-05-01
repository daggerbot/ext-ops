/*
 * Copyright (c) 2023 Martin Mills <daggerbot@gmail.com>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use crate::error::{ArithmeticError, Overflow, RangeError, Undefined, Underflow};

/// Checked addition operator which returns a [Result] to indicate success or failure.
pub trait TryAdd<Rhs = Self> {
    type Output;
    type Error;

    fn try_add(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

/// Checked division operator which returns a [Result] to indicate success or failure.
pub trait TryDiv<Rhs = Self> {
    type Output;
    type Error;

    fn try_div(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

/// Checked multiplication operator which returns a [Result] to indicate success or failure.
pub trait TryMul<Rhs = Self> {
    type Output;
    type Error;

    fn try_mul(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

/// Checked negation operator which returns a [Result] to indicate success or failure.
pub trait TryNeg {
    type Output;
    type Error;

    fn try_neg(self) -> Result<Self::Output, Self::Error>;
}

/// Checked remainder operator which returns a [Result] to indicate success or failure.
pub trait TryRem<Rhs = Self> {
    type Output;
    type Error;

    fn try_rem(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

/// Checked subtraction operator which returns a [Result] to indicate success or failure.
pub trait TrySub<Rhs = Self> {
    type Output;
    type Error;

    fn try_sub(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

//--------------------------------------------------------------------------------------------------

/// Implements unary operators for reference types.
macro_rules! impl_unary_ref_ops {
    { $(impl $trait:ident::$fn:ident for $ty:ident;)* } => { $(
        impl<'a> $trait for &'a $ty {
            type Output = $ty;
            type Error = <$ty as $trait>::Error;

            fn $fn(self) -> Result<$ty, Self::Error> {
                $trait::$fn(*self)
            }
        }
    )* };
}

/// Implements binary operators for reference types.
macro_rules! impl_binary_ref_ops {
    { $(impl $trait:ident::$fn:ident for $ty:ident;)* } => { $(
        impl<'a> $trait<$ty> for &'a $ty {
            type Output = $ty;
            type Error = <$ty as $trait>::Error;

            fn $fn(self, rhs: $ty) -> Result<$ty, Self::Error> {
                $trait::$fn(*self, rhs)
            }
        }

        impl<'r> $trait<&'r $ty> for $ty {
            type Output = $ty;
            type Error = <$ty as $trait>::Error;

            fn $fn(self, rhs: &'r $ty) -> Result<$ty, Self::Error> {
                $trait::$fn(self, *rhs)
            }
        }

        impl<'a, 'r> $trait<&'r $ty> for &'a $ty {
            type Output = $ty;
            type Error = <$ty as $trait>::Error;

            fn $fn(self, rhs: &'r $ty) -> Result<$ty, Self::Error> {
                $trait::$fn(*self, *rhs)
            }
        }
    )* };
}

/// Implements checked operators for signed integer types.
macro_rules! impl_int_ops {
    ($($ty:ident),*) => { $(
        impl TryAdd for $ty {
            type Output = $ty;
            type Error = RangeError;

            fn try_add(self, rhs: $ty) -> Result<$ty, RangeError> {
                match self.checked_add(rhs) {
                    None => Err(if self >= 0 {
                        RangeError::Overflow
                    } else {
                        RangeError::Underflow
                    }),
                    Some(n) => Ok(n),
                }
            }
        }

        impl TryDiv for $ty {
            type Output = $ty;
            type Error = ArithmeticError;

            fn try_div(self, rhs: $ty) -> Result<$ty, ArithmeticError> {
                match self.checked_div(rhs) {
                    None => Err(if rhs == 0 {
                        ArithmeticError::Undefined
                    } else {
                        // Only reachable if self == $ty::MIN && rhs == -1.
                        ArithmeticError::Overflow
                    }),
                    Some(n) => Ok(n),
                }
            }
        }

        impl TryMul for $ty {
            type Output = $ty;
            type Error = RangeError;

            fn try_mul(self, rhs: $ty) -> Result<$ty, RangeError> {
                match self.checked_mul(rhs) {
                    None => Err(if (self >= 0) == (rhs >= 0) {
                        RangeError::Overflow
                    } else {
                        RangeError::Underflow
                    }),
                    Some(n) => Ok(n),
                }
            }
        }

        impl TryNeg for $ty {
            type Output = $ty;
            type Error = Overflow;

            fn try_neg(self) -> Result<$ty, Overflow> {
                match self.checked_neg() {
                    None => Err(Overflow),
                    Some(n) => Ok(n),
                }
            }
        }

        impl TryRem for $ty {
            type Output = $ty;
            type Error = Undefined;

            fn try_rem(self, rhs: $ty) -> Result<$ty, Undefined> {
                match self.checked_rem(rhs) {
                    None => if rhs == 0 {
                        Err(Undefined)
                    } else {
                        // Only reachable if self == $ty::MIN && rhs == -1. Accepted because we know
                        // what the result would be if division did not result in an overflow.
                        Ok(0)
                    },
                    Some(n) => Ok(n),
                }
            }
        }

        impl TrySub for $ty {
            type Output = $ty;
            type Error = RangeError;

            fn try_sub(self, rhs: $ty) -> Result<$ty, RangeError> {
                match self.checked_sub(rhs) {
                    None => Err(if self >= 0 {
                        RangeError::Overflow
                    } else {
                        RangeError::Underflow
                    }),
                    Some(n) => Ok(n),
                }
            }
        }

        impl_unary_ref_ops! {
            impl TryNeg::try_neg for $ty;
        }

        impl_binary_ref_ops! {
            impl TryAdd::try_add for $ty;
            impl TryDiv::try_div for $ty;
            impl TryMul::try_mul for $ty;
            impl TryRem::try_rem for $ty;
            impl TrySub::try_sub for $ty;
        }
    )* };
}

impl_int_ops!(i8, i16, i32, i64, i128, isize);

/// Implements checked operators for unsigned integer types.
macro_rules! impl_uint_ops {
    ($($ty:ident),*) => { $(
        impl TryAdd for $ty {
            type Output = $ty;
            type Error = Overflow;

            fn try_add(self, rhs: $ty) -> Result<$ty, Overflow> {
                match self.checked_add(rhs) {
                    None => Err(Overflow),
                    Some(n) => Ok(n),
                }
            }
        }

        impl TryDiv for $ty {
            type Output = $ty;
            type Error = Undefined;

            fn try_div(self, rhs: $ty) -> Result<$ty, Undefined> {
                match self.checked_div(rhs) {
                    None => Err(Undefined),
                    Some(n) => Ok(n),
                }
            }
        }

        impl TryMul for $ty {
            type Output = $ty;
            type Error = Overflow;

            fn try_mul(self, rhs: $ty) -> Result<$ty, Overflow> {
                match self.checked_mul(rhs) {
                    None => Err(Overflow),
                    Some(n) => Ok(n),
                }
            }
        }

        impl TryNeg for $ty {
            type Output = $ty;
            type Error = Underflow;

            fn try_neg(self) -> Result<$ty, Underflow> {
                match self.checked_neg() {
                    None => Err(Underflow),
                    Some(n) => Ok(n),
                }
            }
        }

        impl TryRem for $ty {
            type Output = $ty;
            type Error = Undefined;

            fn try_rem(self, rhs: $ty) -> Result<$ty, Undefined> {
                match self.checked_rem(rhs) {
                    None => Err(Undefined),
                    Some(n) => Ok(n),
                }
            }
        }

        impl TrySub for $ty {
            type Output = $ty;
            type Error = Underflow;

            fn try_sub(self, rhs: $ty) -> Result<$ty, Underflow> {
                match self.checked_sub(rhs) {
                    None => Err(Underflow),
                    Some(n) => Ok(n),
                }
            }
        }

        impl_unary_ref_ops! {
            impl TryNeg::try_neg for $ty;
        }

        impl_binary_ref_ops! {
            impl TryAdd::try_add for $ty;
            impl TryDiv::try_div for $ty;
            impl TryMul::try_mul for $ty;
            impl TryRem::try_rem for $ty;
            impl TrySub::try_sub for $ty;
        }
    )* };
}

impl_uint_ops!(u8, u16, u32, u64, u128, usize);

//--------------------------------------------------------------------------------------------------

#[test]
fn test_try_add() {
    assert_eq!(i8::try_add(100, 27), Ok(127));
    assert_eq!(i8::try_add(100, 28), Err(RangeError::Overflow));
    assert_eq!(i8::try_add(-100, -28), Ok(-128));
    assert_eq!(i8::try_add(-100, -29), Err(RangeError::Underflow));
    assert_eq!(u8::try_add(200, 55), Ok(255));
    assert_eq!(u8::try_add(200, 56), Err(Overflow));
}

#[test]
fn test_try_div() {
    assert_eq!(i8::try_div(100, 10), Ok(10));
    assert_eq!(i8::try_div(100, 0), Err(ArithmeticError::Undefined));
    assert_eq!(i8::try_div(-128, -1), Err(ArithmeticError::Overflow));
    assert_eq!(u8::try_div(100, 10), Ok(10));
    assert_eq!(u8::try_div(100, 0), Err(Undefined));
}

#[test]
fn test_try_mul() {
    assert_eq!(i8::try_mul(15, 8), Ok(120));
    assert_eq!(i8::try_mul(16, 8), Err(RangeError::Overflow));
    assert_eq!(i8::try_mul(16, -8), Ok(-128));
    assert_eq!(i8::try_mul(43, -3), Err(RangeError::Underflow));
    assert_eq!(i8::try_mul(-127, -1), Ok(127));
    assert_eq!(i8::try_mul(-128, -1), Err(RangeError::Overflow));
    assert_eq!(u8::try_mul(85, 3), Ok(255));
    assert_eq!(u8::try_mul(16, 16), Err(Overflow));
}

#[test]
fn test_try_neg() {
    assert_eq!(i8::try_neg(127), Ok(-127));
    assert_eq!(i8::try_neg(-128), Err(Overflow));
    assert_eq!(u8::try_neg(0), Ok(0));
    assert_eq!(u8::try_neg(1), Err(Underflow));
}

#[test]
fn test_try_rem() {
    assert_eq!(i8::try_rem(99, 10), Ok(9));
    assert_eq!(i8::try_rem(99, -10), Ok(9));
    assert_eq!(i8::try_rem(-99, 10), Ok(-9));
    assert_eq!(i8::try_rem(-99, -10), Ok(-9));
    assert_eq!(i8::try_rem(-128, -1), Ok(0)); // Division would overflow.
    assert_eq!(i8::try_rem(99, 0), Err(Undefined));
    assert_eq!(u8::try_rem(99, 10), Ok(9));
    assert_eq!(u8::try_rem(99, 0), Err(Undefined));
}

#[test]
fn test_try_sub() {
    assert_eq!(i8::try_sub(0, -127), Ok(127));
    assert_eq!(i8::try_sub(0, -128), Err(RangeError::Overflow));
    assert_eq!(i8::try_sub(-1, 127), Ok(-128));
    assert_eq!(i8::try_sub(-2, 127), Err(RangeError::Underflow));
    assert_eq!(u8::try_sub(100, 100), Ok(0));
    assert_eq!(u8::try_sub(0, 1), Err(Underflow));
}
