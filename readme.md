# UnitRanged

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![no_std](https://img.shields.io/badge/no__std-compatible-brightgreen.svg)](https://github.com/rust-embedded/wg)

A lightweight, `no_std`-compatible fixed-point type representing values in the range `[0, 1)` using 32-bit precision.

 The code is formatted according to the [M-RUST](https://github.com/DanilaMint/formating/blob/main/rust.md) standard.

## Overview

`UnitRanged` is a compact fixed-point type that maps the interval `[0, 1)` onto the full range of `u32`. This provides:

- **32-bit precision**: Step size of `1/2^32 ≈ 2.33e-10`
- **`no_std` compatible**: Works without the standard library
- **Fast conversions**: Optimized bit-twiddling for `f32`/`f64` conversions
- **Efficient arithmetic**: Multiplication implemented via 64-bit intermediate

## Features

- ✅ `no_std` ready
- ✅ Constant-time conversions
- ✅ Zero-cost abstractions
- ✅ Precise bit-level control
- ✅ Fast multiplication with saturation

## Installation

This crate is for personal use and not published on [crates.io](https://crates.io). To use it, add this to your `Cargo.toml`:

```toml
unit_ranged = { git = "https://github.com/DanilaMint/unit_ranged" }
```

or for local development:

```toml
unit_ranged = { path = "./path/to/lib/unit_ranged" }
```

## Usage

```rust
use unit_ranged::UnitRanged;

// Create from f32/f64
let a = UnitRanged::from_f32(0.5);
let b = UnitRanged::from_f64(0.75);

// Convert back to float
assert_eq!(a.to_f32(), 0.5);
assert_eq!(b.to_f64(), 0.75);

// Multiplication
let c = a * b;  // 0.5 * 0.75 = 0.375
assert!((c.to_f64() - 0.375).abs() < 1e-9);

// Raw access
let raw = a.into_raw();  // 2147483648 (2^31)
let d = UnitRanged::from_raw(raw);
assert_eq!(a, d);

// Constants
assert_eq!(UnitRanged::MIN.to_f64(), 0.0);
assert_eq!(UnitRanged::MAX.to_f64(), 1.0);
assert_eq!(UnitRanged::HALF.to_f64(), 0.5);
assert_eq!(UnitRanged::EPSILON.to_f64(), 2.3283064365386963e-10);
```

