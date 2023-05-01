/*
 * Copyright (c) 2023 Martin Mills <daggerbot@gmail.com>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

/// Addition operator which wraps around the type's boundaries in case of overflow or underflow.
pub trait WrappingAdd<Rhs = Self> {
    type Output;
    fn wrapping_add(self, rhs: Rhs) -> Self::Output;
}

/// Multiplication operator which wraps around the type's boundaries in case of overflow or
/// underflow.
pub trait WrappingMul<Rhs = Self> {
    type Output;
    fn wrapping_mul(self, rhs: Rhs) -> Self::Output;
}

/// Negation operator which wraps around the type's boundaries in case of overflow or underflow.
pub trait WrappingNeg {
    type Output;
    fn wrapping_neg(self) -> Self::Output;
}

/// Subtraction operator which wraps around the type's boundaries in case of overflow or underflow.
pub trait WrappingSub<Rhs = Self> {
    type Output;
    fn wrapping_sub(self, rhs: Rhs) -> Self::Output;
}

//--------------------------------------------------------------------------------------------------

/// Implements unary wrapping operators.
macro_rules! impl_unary_ops {
    { $(impl $trait:ident::$fn:ident for $ty:ident;)* } => { $(
        impl $trait for $ty {
            type Output = $ty;

            fn $fn(self) -> $ty {
                self.$fn()
            }
        }

        impl<'a> $trait for &'a $ty {
            type Output = $ty;

            fn $fn(self) -> $ty {
                $trait::$fn(*self)
            }
        }
    )* };
}

/// Implements binary wrapping operators.
macro_rules! impl_binary_ops {
    { $(impl $trait:ident::$fn:ident for $ty:ident;)* } => { $(
        impl $trait for $ty {
            type Output = $ty;

            fn $fn(self, rhs: $ty) -> $ty {
                self.$fn(rhs)
            }
        }

        impl<'a> $trait<$ty> for &'a $ty {
            type Output = $ty;

            fn $fn(self, rhs: $ty) -> $ty {
                $trait::$fn(*self, rhs)
            }
        }

        impl<'r> $trait<&'r $ty> for $ty {
            type Output = $ty;

            fn $fn(self, rhs: &'r $ty) -> $ty {
                $trait::$fn(self, *rhs)
            }
        }

        impl<'a, 'r> $trait<&'r $ty> for &'a $ty {
            type Output = $ty;

            fn $fn(self, rhs: &'r $ty) -> $ty {
                $trait::$fn(*self, *rhs)
            }
        }
    )* };
}

/// Implements operators for integer types.
macro_rules! impl_int_ops {
    ($($ty:ident),*) => { $(
        impl_unary_ops! {
            impl WrappingNeg::wrapping_neg for $ty;
        }

        impl_binary_ops! {
            impl WrappingAdd::wrapping_add for $ty;
            impl WrappingMul::wrapping_mul for $ty;
            impl WrappingSub::wrapping_sub for $ty;
        }
    )* };
}

impl_int_ops!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

//--------------------------------------------------------------------------------------------------

#[test]
fn test_wrapping_add() {
    assert_eq!(WrappingAdd::wrapping_add(100i8, 27), 127);
    assert_eq!(WrappingAdd::wrapping_add(100i8, 28), -128);
    assert_eq!(WrappingAdd::wrapping_add(-100i8, -28), -128);
    assert_eq!(WrappingAdd::wrapping_add(-100i8, -29), 127);
    assert_eq!(WrappingAdd::wrapping_add(200u8, 55), 255);
    assert_eq!(WrappingAdd::wrapping_add(200u8, 56), 0);
}

#[test]
fn test_wrapping_mul() {
    assert_eq!(WrappingMul::wrapping_mul(8i8, 15), 120);
    assert_eq!(WrappingMul::wrapping_mul(8i8, 16), -128);
    assert_eq!(WrappingMul::wrapping_mul(8i8, -16), -128);
    assert_eq!(WrappingMul::wrapping_mul(3i8, -43), 127);
    assert_eq!(WrappingMul::wrapping_mul(-1i8, -127), 127);
    assert_eq!(WrappingMul::wrapping_mul(-1i8, -128), -128);
    assert_eq!(WrappingMul::wrapping_mul(85u8, 3), 255);
    assert_eq!(WrappingMul::wrapping_mul(16u8, 16), 0);
}

#[test]
fn test_wrapping_neg() {
    assert_eq!(WrappingNeg::wrapping_neg(127i8), -127);
    assert_eq!(WrappingNeg::wrapping_neg(-127i8), 127);
    assert_eq!(WrappingNeg::wrapping_neg(-128i8), -128);
    assert_eq!(WrappingNeg::wrapping_neg(0u8), 0);
    assert_eq!(WrappingNeg::wrapping_neg(1u8), 255);
    assert_eq!(WrappingNeg::wrapping_neg(255u8), 1);
}

#[test]
fn test_wrapping_sub() {
    assert_eq!(WrappingSub::wrapping_sub(100i8, -27), 127);
    assert_eq!(WrappingSub::wrapping_sub(100i8, -28), -128);
    assert_eq!(WrappingSub::wrapping_sub(-100i8, 28), -128);
    assert_eq!(WrappingSub::wrapping_sub(-100i8, 29), 127);
    assert_eq!(WrappingSub::wrapping_sub(100u8, 100), 0);
    assert_eq!(WrappingSub::wrapping_sub(100u8, 101), 255);
}
