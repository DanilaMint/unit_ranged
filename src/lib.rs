// (M-RUST)

#![cfg_attr(not(test), no_std)]

mod unit;

#[cfg(test)]
mod tests;

pub use unit::UnitRanged;
