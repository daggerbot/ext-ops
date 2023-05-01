/*
 * Copyright (c) 2023 Martin Mills <daggerbot@gmail.com>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#![cfg_attr(not(feature = "std"), no_std)]

//! General purpose arithmetic operator traits which are missing from the standard library.

mod error;
mod saturating_ops;
mod try_ops;
mod wrapping_ops;

pub use error::{
    ArithmeticError,
    Overflow,
    RangeError,
    Undefined,
    Underflow,
};
pub use saturating_ops::{
    SaturatingAdd,
    SaturatingMul,
    SaturatingNeg,
    SaturatingSub,
};
pub use try_ops::{
    TryAdd,
    TryDiv,
    TryMul,
    TryNeg,
    TryRem,
    TrySub,
};
pub use wrapping_ops::{
    WrappingAdd,
    WrappingMul,
    WrappingNeg,
    WrappingSub,
};
