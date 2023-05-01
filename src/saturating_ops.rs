/*
 * Copyright (c) 2023 Martin Mills <daggerbot@gmail.com>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

/// Addition operator which returns the closest possible value in the event of an overflow or
/// underflow.
pub trait SaturatingAdd<Rhs = Self> {
    type Output;
    fn saturating_add(self, rhs: Rhs) -> Self::Output;
}

/// Multiplication operator which returns the closest possible value in the event of an overflow or
/// underflow.
pub trait SaturatingMul<Rhs = Self> {
    type Output;
    fn saturating_mul(self, rhs: Rhs) -> Self::Output;
}

/// Negation operator which returns the closest possible value in the event of an overflow or
/// underflow.
pub trait SaturatingNeg {
    type Output;
    fn saturating_neg(self) -> Self::Output;
}

/// Subtraction operator which returns the closest possible value in the event of an overflow or
/// underflow.
pub trait SaturatingSub<Rhs = Self> {
    type Output;
    fn saturating_sub(self, rhs: Rhs) -> Self::Output;
}

//--------------------------------------------------------------------------------------------------

/// Implements unary operators for reference types.
macro_rules! impl_unary_ref_ops {
    { $(impl $trait:ident::$fn:ident for $ty:ident;)* } => { $(
        impl<'a> $trait for &'a $ty {
            type Output = $ty;

            fn $fn(self) -> $ty {
                $trait::$fn(*self)
            }
        }
    )* };
}

/// Implements binary saturating operators.
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

/// Implements saturating operators for signed integer types.
macro_rules! impl_int_ops {
    ($($ty:ident),*) => { $(
        impl SaturatingNeg for $ty {
            type Output = $ty;

            fn saturating_neg(self) -> $ty {
                self.saturating_neg()
            }
        }

        impl_unary_ref_ops! {
            impl SaturatingNeg::saturating_neg for $ty;
        }

        impl_binary_ops! {
            impl SaturatingAdd::saturating_add for $ty;
            impl SaturatingMul::saturating_mul for $ty;
            impl SaturatingSub::saturating_sub for $ty;
        }
    )* };
}

impl_int_ops!(i8, i16, i32, i64, i128, isize);

/// Implements saturating operators for unsigned integer types.
macro_rules! impl_uint_ops {
    ($($ty:ident),*) => { $(
        impl_binary_ops! {
            impl SaturatingAdd::saturating_add for $ty;
            impl SaturatingMul::saturating_mul for $ty;
            impl SaturatingSub::saturating_sub for $ty;
        }
    )* };
}

impl_uint_ops!(u8, u16, u32, u64, u128, usize);

//--------------------------------------------------------------------------------------------------

#[test]
fn test_saturating_add() {
    assert_eq!(SaturatingAdd::saturating_add(100i8, 26), 126);
    assert_eq!(SaturatingAdd::saturating_add(100i8, 27), 127);
    assert_eq!(SaturatingAdd::saturating_add(100i8, 28), 127);
    assert_eq!(SaturatingAdd::saturating_add(-100i8, -27), -127);
    assert_eq!(SaturatingAdd::saturating_add(-100i8, -28), -128);
    assert_eq!(SaturatingAdd::saturating_add(-100i8, -29), -128);
    assert_eq!(SaturatingAdd::saturating_add(200u8, 54), 254);
    assert_eq!(SaturatingAdd::saturating_add(200u8, 55), 255);
    assert_eq!(SaturatingAdd::saturating_add(200u8, 56), 255);
}

#[test]
fn test_saturating_mul() {
    assert_eq!(SaturatingMul::saturating_mul(50i8, 2), 100);
    assert_eq!(SaturatingMul::saturating_mul(50i8, 3), 127);
    assert_eq!(SaturatingMul::saturating_mul(50i8, -2), -100);
    assert_eq!(SaturatingMul::saturating_mul(50i8, -3), -128);
    assert_eq!(SaturatingMul::saturating_mul(-50i8, -2), 100);
    assert_eq!(SaturatingMul::saturating_mul(-50i8, -3), 127);
    assert_eq!(SaturatingMul::saturating_mul(50u8, 5), 250);
    assert_eq!(SaturatingMul::saturating_mul(50u8, 6), 255);
}

#[test]
fn test_saturating_neg() {
    assert_eq!(SaturatingNeg::saturating_neg(127i8), -127);
    assert_eq!(SaturatingNeg::saturating_neg(-127i8), 127);
    assert_eq!(SaturatingNeg::saturating_neg(-128i8), 127);
}

#[test]
fn test_saturating_sub() {
    assert_eq!(SaturatingSub::saturating_sub(100i8, -26), 126);
    assert_eq!(SaturatingSub::saturating_sub(100i8, -27), 127);
    assert_eq!(SaturatingSub::saturating_sub(100i8, -28), 127);
    assert_eq!(SaturatingSub::saturating_sub(-100i8, 27), -127);
    assert_eq!(SaturatingSub::saturating_sub(-100i8, 28), -128);
    assert_eq!(SaturatingSub::saturating_sub(-100i8, 29), -128);
    assert_eq!(SaturatingSub::saturating_sub(100u8, 99), 1);
    assert_eq!(SaturatingSub::saturating_sub(100u8, 100), 0);
    assert_eq!(SaturatingSub::saturating_sub(100u8, 101), 0);
}
