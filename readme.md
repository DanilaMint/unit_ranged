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
- **Const-ready**: Full `const` support for conversions and operations
- **num-traits integration**: Complete implementation of numeric traits
- **Efficient arithmetic**: Hardware-accelerated operations with proper overflow handling

## Features

- ✅ `no_std` ready
- ✅ `const` conversions (bitwise and FPU-based)
- ✅ Zero-cost abstractions
- ✅ Precise bit-level control
- ✅ Full `num-traits` support
- ✅ Checked/wrapping/saturating arithmetic
- ✅ Byte-level serialization/deserialization

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

// Create from f32/f64 with clamping
let a = UnitRanged::from_f32_clamped(0.5);
let b = UnitRanged::from_f64_clamped(0.75);

// Const conversions
const ZERO: UnitRanged = UnitRanged::from_f32_clamped_const(0.0);
const HALF: UnitRanged = UnitRanged::HALF;

// Unsafe conversions (no bounds checking)
let x = unsafe { UnitRanged::from_f32_unchecked(0.5) };
let y = unsafe { UnitRanged::from_f64_unchecked(0.75) };

// Convert back to float (const bitwise)
assert_eq!(a.to_f32_const(), 0.5);
assert_eq!(b.to_f64_const(), 0.75);

// FPU-based conversions
let f32_val = a.to_f32_fpu();
let f64_val = b.to_f64_fpu();

// Arithmetic operations
let c = a * b;  // 0.5 * 0.75 = 0.375
let sum = a + b;
let diff = b - a;

// Raw access
let raw = a.into_raw();  // 2147483648 (2^31)
let d = UnitRanged::from_raw(raw);
assert_eq!(a, d);

// Constants
assert_eq!(UnitRanged::MIN.to_f64_const(), 0.0);
assert_eq!(UnitRanged::MAX.to_f64_const(), 1.0);
assert_eq!(UnitRanged::HALF.to_f64_const(), 0.5);
assert_eq!(UnitRanged::EPSILON.to_f64_const(), 2.3283064365386963e-10);
```

## num-traits Integration

```rust
use num_traits::{FromPrimitive, ToPrimitive, Bounded, CheckedAdd};
use unit_ranged::UnitRanged;

// FromPrimitive - returns None for out-of-range or NaN
let x = UnitRanged::from_f32(0.5).unwrap();
let invalid = UnitRanged::from_f32(1.5); // None (> 1)
let nan = UnitRanged::from_f32(f32::NAN); // None

// ToPrimitive
let f32_val: Option<f32> = x.to_f32();
let int_val: Option<u32> = x.to_u32(); // 0 or 1 only

// Bounded
assert_eq!(UnitRanged::min_value(), UnitRanged::MIN);
assert_eq!(UnitRanged::max_value(), UnitRanged::MAX);

// Checked arithmetic
let a = UnitRanged::from(0.5_f32);
let b = UnitRanged::from(0.6_f32);
let checked = a.checked_add(&b); // Option<UnitRanged>
```

## Conversion Performance

- **from_f32_clamped_const**: ~10-15 instructions (bitwise, const-friendly)
- **from_f32_unchecked**: ~1-2 instructions (FPU multiply)
- **to_f32_const**: ~15-20 instructions (bitwise, const-friendly)  
- **to_f32_fpu**: ~2 instructions (FPU multiply)

## Implemented Traits

### Standard
- `Copy`, `Clone`, `Default`, `Eq`, `PartialEq`, `Hash`
- `Ord`, `PartialOrd`, `Debug`, `Display`
- `Add`, `Sub`, `Mul`, `Div`, `Rem`
- `From<T>` for f32, f64, u32
- `Into<T>` for f32, f64, u32

### num-traits
- `Bounded`, `FromPrimitive`, `ToPrimitive`
- `FromBytes`, `ToBytes`
- `Checked*` (Add, Sub, Mul, Div, Rem)
- `Wrapping*` (Add, Sub, Mul)
- `Saturating*` (Add, Sub, Mul)

