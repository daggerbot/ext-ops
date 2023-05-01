# ext-ops

Copyright (c) 2023 Martin Mills [<daggerbot@gmail.com>]

[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)

General purpose arithmetic operator traits for Rust which are missing from the standard library.
The name `ext-ops` is short for "extension operators."

This crate is intended to address shortcomings in parts of the [`num-traits`](https://crates.io/crates/num-traits) crate:
* The `Try*` traits provided by `ext-ops` return a `Result` instead of an `Option`, so arithmetic errors can be propagated with the `?` operator.
* `ext-ops` traits do not require operands to be references. This allows for optimizations that may not be possible using `num-traits`, such as consuming a `BigInt` operand instead of constructing a new one.
* `ext-ops` traits do not have trait constraints. For example, `TryAdd` can be implemented for a type that does not implement `Add`.

At the time of writing, some expected traits may not be implemented for some types or even defined at all.
This is because it has not yet been decided which of multiple possible behaviors is best.
Feel free to open an issue if something you desire is missing.
