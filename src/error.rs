/*
 * Copyright (c) 2023 Martin Mills <daggerbot@gmail.com>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use core::fmt::{Display, Formatter};

/// Error raised when a checked arithmetic operation fails.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ArithmeticError {
    Undefined,
    Underflow,
    Overflow,
}

impl ArithmeticError {
    fn brief(self) -> &'static str {
        match self {
            ArithmeticError::Undefined => Undefined::BRIEF,
            ArithmeticError::Underflow => Underflow::BRIEF,
            ArithmeticError::Overflow => Overflow::BRIEF,
        }
    }
}

impl Display for ArithmeticError {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        f.write_str(self.brief())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ArithmeticError {
    fn description(&self) -> &str { self.brief() }
}

/// Error raised when the result of a checked arithmetic operation is too high to be represented by
/// the destination type.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Overflow;

impl Overflow {
    const BRIEF: &'static str = "arithmetic overflow";
}

impl Display for Overflow {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        f.write_str(Overflow::BRIEF)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Overflow {
    fn description(&self) -> &str { Overflow::BRIEF }
}

/// Error raised when the result of a checked arithmetic operation is too high or too low to be
/// represented by the destination type.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RangeError {
    Underflow,
    Overflow,
}

impl RangeError {
    fn brief(self) -> &'static str {
        match self {
            RangeError::Underflow => Underflow::BRIEF,
            RangeError::Overflow => Overflow::BRIEF,
        }
    }
}

impl Display for RangeError {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        f.write_str(self.brief())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for RangeError {
    fn description(&self) -> &str { self.brief() }
}

/// Error raised when the result of a checked arithmetic operation is undefined, indeterminate, or
/// not a number.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Undefined;

impl Undefined {
    const BRIEF: &'static str = "arithmetic result undefined";
}

impl Display for Undefined {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        f.write_str(Undefined::BRIEF)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Undefined {
    fn description(&self) -> &str { Undefined::BRIEF }
}

/// Error raised when the result of a checked arithmetic operation is too low to be represented by
/// the destination type.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Underflow;

impl Underflow {
    const BRIEF: &'static str = "arithmetic underflow";
}

impl Display for Underflow {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        f.write_str(Underflow::BRIEF)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Underflow {
    fn description(&self) -> &str { Underflow::BRIEF }
}
