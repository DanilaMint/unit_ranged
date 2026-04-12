// (M-RUST)

use num_traits::{
    Bounded, FromPrimitive, ToPrimitive,
    FromBytes, ToBytes, 
    CheckedAdd, CheckedSub, CheckedMul, CheckedDiv, CheckedRem,
    WrappingAdd, WrappingSub, WrappingMul,
    SaturatingAdd, SaturatingMul, SaturatingSub
};
use core::{
    ops::{Add, Sub, Mul, Div, Rem,},
    fmt::{self, Debug, Display},
    cmp::{Ord, PartialOrd, Ordering}
};

#[repr(transparent)]
#[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct UnitRanged(u32);

impl UnitRanged {
    /// Minimal value `0.0`
    pub const MIN : Self = Self(0);
    /// Maximal value ~`1.0`
    pub const MAX : Self = Self(u32::MAX);
    /// A half `0.5`
    pub const HALF : Self = Self(1 << 31);
    /// Minimal value diffirence in `f64`
    pub const F64_EPSILON : f64 = 1.0 / 4294967296.;
    /// Minimal value diffirence in `f32`
    pub const F32_EPSILON : f32 = 2.32830643e-10;
    /// Minimal value diffirence
    pub const EPSILON : Self = Self(1);
    
    const TWO_POWER_32_F32: f32 = 4294967296.;
    const TWO_POWER_32_F64: f64 = 4294967296.;

    #[inline(always)]
    pub const fn from_raw(x: u32) -> Self {
        Self(x)
    }

    #[inline(always)]
    pub const fn into_raw(&self) -> u32 {
        self.0
    }

    /// Makes `UnitRanged` from `f32`
    /// - using bit-operations
    /// - const
    /// - clamping value to [0; 1)
    /// - if x is NaN, it returns 0
    #[inline]
    pub const fn from_f32_clamped_const(x: f32) -> Self {
        let bits = x.to_bits();

        // Check for NaN, negative, or too small values in one go
        // NaN: exponent = 0xFF AND mantissa != 0
        // Negative: sign bit = 1
        // Too small: value < F32_EPSILON
        let is_nan = (bits & 0x7FFFFFFF) > 0x7F800000;
        let is_negative = (bits & 0x80000000) != 0;
        let is_too_small = x < Self::F32_EPSILON;

        if is_nan || is_negative || is_too_small {
            return Self::MIN;
        }

        if x >= 1.0 {
            return Self::MAX;
        }

        unsafe { Self::from_f32_unchecked_const(x) }
    }

    /// Makes `UnitRanged` from `f64`
    /// - using bit-operations
    /// - const
    /// - clamping value to [0; 1)
    /// - if x is NaN, it returns 0
    #[inline]
    pub const fn from_f64_clamped_const(x: f64) -> Self {
        let bits = x.to_bits();

        // Check for NaN, negative, or too small values in one go
        // NaN: exponent = 0x7FF AND mantissa != 0
        // Negative: sign bit = 1
        // Too small: value < F64_EPSILON
        let is_nan = (bits & 0x7FFFFFFFFFFFFFFF) > 0x7FF0000000000000;
        let is_negative = (bits & 0x8000000000000000) != 0;
        let is_too_small = x < Self::F64_EPSILON;

        if is_nan || is_negative || is_too_small {
            return Self::MIN
        }

        if x >= 1.0 {
            return Self::MAX
        }

        unsafe { Self::from_f64_unchecked_const(x) }
    }

    /// Makes `UnitRanged` from `f32`
    /// - using bit-operations
    /// - const
    /// 
    /// # Safety
    /// - x must be in [0; 1)
    /// - x must not be NaN
    #[inline]
    pub const unsafe fn from_f32_unchecked_const(x: f32) -> Self {
        let bits = x.to_bits();
        let exp = (bits >> 23) & 0xff;
        let mantissa = bits & 0x007fffff;

        let normalized = (mantissa | (1 << 23)) << 8;

        let shift = 126 - exp;

        let result = normalized >> shift;

        Self(result)
    }

    /// Makes `UnitRanged` from `f64`
    /// - using bit-operations
    /// - const
    /// 
    /// # Safety
    /// - x must be in [0; 1)
    /// - x must not be NaN
    #[inline]
    pub const unsafe fn from_f64_unchecked_const(x: f64) -> Self {
        let bits = x.to_bits();
        let exp = (bits >> 52) & 0x7ff;
        let mantissa = bits & 0x000fffffffffffff;

        let normalized = ((mantissa | 1 << 52) >> 21) as u32;

        let shift = 1022 - exp;

        let result = normalized >> shift;

        Self(result)
    }

    /// Makes `UnitRanged` from `f32`
    /// 
    /// # Safety
    /// - x must be in [0; 1)
    /// - x must not be NaN
    #[inline]
    pub unsafe fn from_f32_unchecked(x: f32) -> Self {
        let prod = x * Self::TWO_POWER_32_F32;
        Self(unsafe { prod.to_int_unchecked() })
    }

    /// Makes `UnitRanged` from `f64`
    /// 
    /// # Safety
    /// - x must be in [0; 1)
    /// - x must not be NaN
    #[inline]
    pub unsafe fn from_f64_unchecked(x: f64) -> Self {
        let prod = x * Self::TWO_POWER_32_F64;
        Self(unsafe { prod.to_int_unchecked() })
    }

    /// Makes `UnitRanged` from `f32`
    /// - clamping x to [0; 1)
    /// - if x is NaN, returns 0
    #[inline]
    pub fn from_f32_clamped(x: f32) -> Self {
        if x.is_nan() {
            Self::MIN
        } else {
            unsafe { Self::from_f32_unchecked(x.clamp(0., 1.)) }
        }
    }

    /// Makes `UnitRanged` from `f32`
    /// - clamping x to [0; 1)
    /// - if x is NaN, returns 0
    #[inline]
    pub fn from_f64_clamped(x: f64) -> Self {
        if x.is_nan() {
            Self::MIN
        } else {
            unsafe { Self::from_f64_unchecked(x.clamp(0., 1.)) }
        }
    }

    /// Makes `f32` from `UnitRanged`
    #[inline]
    pub const fn to_f32_const(&self) -> f32 {
        let n = self.0;
        if n == 0 {
            return 0.0
        }

        let lz = n.leading_zeros();

        let implicit_mask = 1 << (31 - lz);

        let mantissa = (n ^ implicit_mask) << lz >> 8;

        let exp = (126 - lz) << 23;

        let result = exp | mantissa;
        
        f32::from_bits(result)
    }

    /// Makes `f64` from `UnitRanged`
    /// - May have a number error
    #[inline]
    pub const fn to_f64_const(&self) -> f64 {
        let n64 = self.0 as u64;
        if n64 == 0 {
            return 0.0;
        }

        let lz = self.0.leading_zeros();

        let implicit_mask = 1u64 << (31 - lz);

        let mantissa = (n64 ^ implicit_mask) << (lz + 32) >> 11;

        let exp = (1022 - lz as u64) << 52;

        let result = exp | mantissa;

        f64::from_bits(result)
    }

    #[inline]
    pub fn to_f32_fpu(&self) -> f32 {
        let val = self.0 as f32;
        val * Self::F32_EPSILON
    }
    
    #[inline]
    pub fn to_f64_fpu(&self) -> f64 {
        let val = self.0 as f64;
        val * Self::F64_EPSILON
    }

    /// Multiplicate two `UnitRanged`
    ///
    /// May be error while mul, there is should use:
    /// ```rust
    /// # use unit_ranged::UnitRanged;
    /// let a = UnitRanged::from(0.5_f32);
    /// let b = UnitRanged::from(0.25_f32);
    /// // For more accurate results:
    /// let result = a.to_f32_const() * b.to_f32_const();
    /// // or
    /// let result = a.to_f64_const() * b.to_f64_const();
    /// # let _ = result;
    /// ```
    #[inline]
    pub const fn _mul(self, other: Self) -> Self {
        let a = self.0 as u64;
        let b = other.0 as u64;
        let product = a * b;

        let x = (product >> 32) as u32;
        
        Self(x)
    }
}

impl PartialOrd for UnitRanged {
    #[inline(always)]
    fn ge(&self, other: &Self) -> bool {
        self.0 >= other.0
    }

    #[inline(always)]
    fn gt(&self, other: &Self) -> bool {
        self.0 > other.0
    }

    #[inline(always)]
    fn le(&self, other: &Self) -> bool {
        self.0 <= other.0
    }

    #[inline(always)]
    fn lt(&self, other: &Self) -> bool {
        self.0 < other.0
    }

    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for UnitRanged {
    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self {
        Self(self.0.clamp(min.0, max.0))
    }

    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }

    #[inline(always)]
    fn max(self, other: Self) -> Self {
        Self(self.0.max(other.0))
    }

    #[inline(always)]
    fn min(self, other: Self) -> Self {
        Self(self.0.min(other.0))
    }
}   

impl Display for UnitRanged {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:9}", self.to_f64_const())
    }
}

impl Debug for UnitRanged {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UnitRanged(0x{:x})", self.0)
    }
}

impl From<f32> for UnitRanged {
    #[inline(always)]
    fn from(value: f32) -> Self {
        Self::from_f32_clamped(value)
    }
}

impl From<f64> for UnitRanged {
    #[inline(always)]
    fn from(value: f64) -> Self {
        Self::from_f64_clamped(value)
    }
}

impl From<u32> for UnitRanged {
    #[inline(always)]
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<UnitRanged> for u32 {
    #[inline(always)]
    fn from(value: UnitRanged) -> Self {
        value.0
    }
}

impl From<UnitRanged> for f32 {
    #[inline(always)]
    fn from(value: UnitRanged) -> Self {
        value.to_f32_const()
    }
}

impl From<UnitRanged> for f64 {
    #[inline(always)]
    fn from(value: UnitRanged) -> Self {
        value.to_f64_const()
    }
}

impl Add for UnitRanged {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for UnitRanged {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for UnitRanged {
    type Output = Self;

    /// Redirect to `UnitRanged::_mul(ohter)`
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self._mul(rhs)
    }
}

impl Div for UnitRanged {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        let div = (self.0 as u64) << 32;
        Self((div / rhs.0 as u64) as u32)
    }
}

impl Rem for UnitRanged {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        let div = (self.0 as u64) << 32;
        Self((div % rhs.0 as u64) as u32)
    }
}

/*** num_traits ***/

impl Bounded for UnitRanged {
    #[inline(always)]
    fn max_value() -> Self { Self::MAX }

    #[inline(always)]
    fn min_value() -> Self { Self::MIN }
}

impl FromBytes for UnitRanged {
    type Bytes = [u8; 4];

    #[inline(always)]
    fn from_be_bytes(bytes: &Self::Bytes) -> Self {
        Self(u32::from_be_bytes(*bytes))
    }

    #[inline(always)]
    fn from_le_bytes(bytes: &Self::Bytes) -> Self {
        Self(u32::from_le_bytes(*bytes))
    }

    #[inline(always)]
    fn from_ne_bytes(bytes: &Self::Bytes) -> Self {
        Self(u32::from_ne_bytes(*bytes))
    }
}

impl ToBytes for UnitRanged {
    type Bytes = [u8; 4];

    #[inline(always)]
    fn to_be_bytes(&self) -> Self::Bytes {
        self.0.to_be_bytes()
    }

    #[inline(always)]
    fn to_le_bytes(&self) -> Self::Bytes {
        self.0.to_le_bytes()
    }

    #[inline(always)]
    fn to_ne_bytes(&self) -> Self::Bytes {
        self.0.to_ne_bytes()
    }
}

impl FromPrimitive for UnitRanged {
    #[inline]
    fn from_f32(n: f32) -> Option<Self> {
        if n.is_nan() {
            None
        } else if n < 0. {
            None
        } else if n > 1. {
            None
        } else {
            Some(unsafe { Self::from_f32_unchecked(n) })
        }
    }

    #[inline]
    fn from_f64(n: f64) -> Option<Self> {
        if n.is_nan() {
            None
        } else if n < 0. {
            None
        } else if n > 1. {
            None
        } else {
            Some(unsafe { Self::from_f64_unchecked(n) })
        }
    }

    #[inline(always)]
    fn from_i128(n: i128) -> Option<Self> {
        match n {
            0 => Some(Self::MIN),
            1 => Some(Self::MAX),
            _ => None
        }
    }

    #[inline(always)]
    fn from_i16(n: i16) -> Option<Self> {
        match n {
            0 => Some(Self::MIN),
            1 => Some(Self::MAX),
            _ => None
        }
    }

    #[inline(always)]
    fn from_i32(n: i32) -> Option<Self> {
        match n {
            0 => Some(Self::MIN),
            1 => Some(Self::MAX),
            _ => None
        }
    }

    #[inline(always)]
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0 => Some(Self::MIN),
            1 => Some(Self::MAX),
            _ => None
        }
    }

    #[inline(always)]
    fn from_i8(n: i8) -> Option<Self> {
        match n {
            0 => Some(Self::MIN),
            1 => Some(Self::MAX),
            _ => None
        }
    }

    #[inline(always)]
    fn from_isize(n: isize) -> Option<Self> {
        match n {
            0 => Some(Self::MIN),
            1 => Some(Self::MAX),
            _ => None
        }
    }

    #[inline(always)]
    fn from_u128(n: u128) -> Option<Self> {
        match n {
            0 => Some(Self::MIN),
            1 => Some(Self::MAX),
            _ => None
        }
    }

    #[inline(always)]
    fn from_u16(n: u16) -> Option<Self> {
        match n {
            0 => Some(Self::MIN),
            1 => Some(Self::MAX),
            _ => None
        }
    }

    #[inline(always)]
    fn from_u32(n: u32) -> Option<Self> {
        match n {
            0 => Some(Self::MIN),
            1 => Some(Self::MAX),
            _ => None
        }
    }

    #[inline(always)]
    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0 => Some(Self::MIN),
            1 => Some(Self::MAX),
            _ => None
        }
    }

    #[inline(always)]
    fn from_u8(n: u8) -> Option<Self> {
        match n {
            0 => Some(Self::MIN),
            1 => Some(Self::MAX),
            _ => None
        }
    }

    #[inline(always)]
    fn from_usize(n: usize) -> Option<Self> {
        match n {
            0 => Some(Self::MIN),
            1 => Some(Self::MAX),
            _ => None
        }
    }
}

impl ToPrimitive for UnitRanged {
    #[inline]
    fn to_f32(&self) -> Option<f32> { Some(self.to_f32_fpu()) }

    #[inline]
    fn to_f64(&self) -> Option<f64> { Some(self.to_f64_fpu()) }

    #[inline]
    fn to_i128(&self) -> Option<i128> { Some((self.0 == u32::MAX).into()) }

    #[inline(always)]
    fn to_i16(&self) -> Option<i16> { Some((self.0 == u32::MAX).into()) }

    #[inline(always)]
    fn to_i32(&self) -> Option<i32> { Some((self.0 == u32::MAX).into()) }

    #[inline(always)]
    fn to_i64(&self) -> Option<i64> { Some((self.0 == u32::MAX).into()) }

    #[inline(always)]
    fn to_i8(&self) -> Option<i8> { Some((self.0 == u32::MAX).into()) }

    #[inline(always)]
    fn to_isize(&self) -> Option<isize> { Some((self.0 == u32::MAX).into()) }

    #[inline(always)]
    fn to_u128(&self) -> Option<u128> { Some((self.0 == u32::MAX).into()) }

    #[inline(always)]
    fn to_u16(&self) -> Option<u16> { Some((self.0 == u32::MAX).into()) }

    #[inline(always)]
    fn to_u32(&self) -> Option<u32> { Some((self.0 == u32::MAX).into()) }

    #[inline(always)]
    fn to_u64(&self) -> Option<u64> { Some((self.0 == u32::MAX).into()) }

    #[inline(always)]
    fn to_u8(&self) -> Option<u8> { Some((self.0 == u32::MAX).into()) }

    #[inline(always)]
    fn to_usize(&self) -> Option<usize> { Some((self.0 == u32::MAX).into()) }
}

impl CheckedAdd for UnitRanged {
    #[inline]
    fn checked_add(&self, v: &Self) -> Option<Self> {
        self.0.checked_add(v.0).map(|x| Self(x))
    }
}

impl CheckedSub for UnitRanged {
    #[inline]
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        self.0.checked_sub(v.0).map(|x| Self(x))
    }
}

impl CheckedMul for UnitRanged {
    #[inline]
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        self.0.checked_mul(v.0).map(|x| Self(x))
    }
}

impl CheckedDiv for UnitRanged {
    #[inline]
    fn checked_div(&self, v: &Self) -> Option<Self> {
        self.0.checked_div(v.0).map(|x| Self(x))
    }
}

impl CheckedRem for UnitRanged {
    #[inline]
    fn checked_rem(&self, v: &Self) -> Option<Self> {
        self.0.checked_rem(v.0).map(|x| Self(x))
    }
}

impl WrappingAdd for UnitRanged {
    #[inline(always)]
    fn wrapping_add(&self, v: &Self) -> Self {
        Self(self.0.wrapping_add(v.0))
    }
}

impl WrappingSub for UnitRanged {
    #[inline(always)]
    fn wrapping_sub(&self, v: &Self) -> Self {
        Self(self.0.wrapping_sub(v.0))
    }
}

impl WrappingMul for UnitRanged {
    #[inline]
    fn wrapping_mul(&self, v: &Self) -> Self {
        self._mul(*v)
    }
}

impl SaturatingAdd for UnitRanged {
    #[inline(always)]
    fn saturating_add(&self, v: &Self) -> Self {
        Self(self.0.saturating_add(v.0))
    }
}

impl SaturatingSub for UnitRanged {
    #[inline(always)]
    fn saturating_sub(&self, v: &Self) -> Self {
        Self(self.0.saturating_sub(v.0))
    }
}

impl SaturatingMul for UnitRanged {
    #[inline]
    fn saturating_mul(&self, v: &Self) -> Self {
        self._mul(*v)
    }
}
