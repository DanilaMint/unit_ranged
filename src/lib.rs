// (M-RUST)

#![cfg_attr(not(test), no_std)]

#[cfg(feature = "serde")]
mod serde;
mod unit;

#[cfg(test)]
mod tests;

pub use unit::UnitRanged;
