// (M-RUST)

use core::{
    ops::{Mul, MulAssign},
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
    pub const F64_EPSILON : f64 = 1.0 / 4294967296.0;
    /// Minimal value diffirence in `f32`
    pub const F32_EPSILON : f32 = 2.3283064365386963e-10;
    /// Minimal value diffirence
    pub const EPSILON : Self = Self(1);

    #[inline(always)]
    pub const fn from_raw(x: u32) -> Self {
        Self(x)
    }

    #[inline(always)]
    pub const fn into_raw(&self) -> u32 {
        self.0
    }

    /// Makes `UnitRanged` from `f32`
    /// - If value is bigger than `1`, value is `UnitRanged::MAX`
    /// - If value is smaller than `0`, value is `UnitRanged::MIN`
    /// - `NaN` value is used as `0`
    #[inline]
    pub const fn from_f32(x: f32) -> Self {
        if x < Self::F32_EPSILON || x.is_nan() {
            return Self::MIN;
        } else if x >= 1.0 {
            return Self::MAX;
        }

        let bits = x.to_bits();

        let exp = (bits >> 23) & 0xff;
        let mantissa = bits & 0x007fffff;

        let normalized = (mantissa | (1 << 23)) << 8;

        let shift = 126 - exp;

        let result = normalized >> shift;

        return Self(result);
    }

    /// Makes `UnitRanged` from `f64`
    /// - If value is bigger than `1`, value is `UnitRanged::MAX`
    /// - If value is smaller than `0`, value is `UnitRanged::MIN`
    /// - `NaN` value is used as `0`
    #[inline]
    pub const fn from_f64(x: f64) -> Self {
        if x < Self::F64_EPSILON || x.is_nan() {
            return Self::MIN
        } else if x >= 1.0 {
            return Self::MAX
        }

        let bits = x.to_bits();
        
        let exp = (bits >> 52) & 0x7ff;
        let mantissa = bits & 0x000fffffffffffff;

        let normalized = ((mantissa | 1 << 52) >> 21) as u32;

        let shift = 1022 - exp;

        let result = normalized >> shift;

        return Self(result)
    }

    /// Makes `f32` from `UnitRanged`
    #[inline]
    pub const fn to_f32(&self) -> f32 {
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
    pub const fn to_f64(&self) -> f64 {
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

    /// Multiplicate two `UnitRanged`
    /// 
    /// May be error while mul, there is should use:
    /// ```rust
    /// a.to_f32() * b.to_f32();
    /// // or
    /// a.to_f64() * b.to_f64();
    /// ```
    #[inline]
    pub const fn _mul(self, other: Self) -> Self {
        let a = self.0 as u64;
        let b = other.0 as u64;
        let product = a * b;
        let max_product_normalize = ((a == u32::MAX as u64) & (b == u32::MAX as u64)) as u64;
        let x = ((product + (1 << 31)) >> 32) + max_product_normalize;
        
        Self(x as u32)
    }

    #[inline]
    pub const fn saturating_add(self, other: Self) -> Self {
        Self(self.0.saturating_add(other.0))
    }

    #[inline]
    pub const fn wrapping_add(self, other: Self) -> Self {
        Self(self.0.wrapping_add(other.0))
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

impl MulAssign for UnitRanged {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self._mul(rhs);
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

    #[inline]
    fn max(self, other: Self) -> Self {
        Self(self.0.max(other.0))
    }

    #[inline]
    fn min(self, other: Self) -> Self {
        Self(self.0.min(other.0))
    }
}   

impl Display for UnitRanged {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:9}", self.to_f64())
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
        Self::from_f32(value)
    }
}

impl From<f64> for UnitRanged {
    #[inline(always)]
    fn from(value: f64) -> Self {
        Self::from_f64(value)
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
        value.to_f32()
    }
}

impl From<UnitRanged> for f64 {
    #[inline(always)]
    fn from(value: UnitRanged) -> Self {
        value.to_f64()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use std::hint::black_box;
    use super::*;

    #[test]
    fn from_f32_test() {
        assert_eq!(UnitRanged::from_f32(0.0), UnitRanged::from_raw(0));
        assert_eq!(UnitRanged::from_f32(1.0), UnitRanged::from_raw(u32::MAX));
        assert_eq!(UnitRanged::from_f32(144.4), UnitRanged::from_raw(u32::MAX));
        assert_eq!(UnitRanged::from_f32(1e-45), UnitRanged::from_raw(0));
        assert_eq!(UnitRanged::from_f32(0.5), UnitRanged::from_raw(u32::MAX / 2 + 1));
        assert_eq!(UnitRanged::from_f32(f32::NAN), UnitRanged::MIN);
    }

    #[test]
    fn test_from_f32_speed() {
        fn from_f32_mul(x : f32) -> UnitRanged {
            const U32_MAX : f32 = u32::MAX as f32;
            let x = x.clamp(0.0, 1.0);
            return UnitRanged::from_raw((x * U32_MAX) as u32);
        }

        const ITERATIONS: u32 = 10_000_000;
        
        let mut results = String::new();
        results.push_str("\n=== F32 CONVERSION BENCHMARK ===\n\n");
        
        // Тестовые значения
        let values = [0.0f32, 0.1, 0.5, 0.999, 1.0, 1.3, 0e-45, -4.1];
        
        for &x in &values {
            results.push_str(&format!("Value: {}\n", x));
            results.push_str(&format!("{}\n", "-".repeat(50)));
            
            // Твой метод
            let start = Instant::now();
            for _ in 0..ITERATIONS {
                black_box(UnitRanged::from_f32(black_box(x)));
            }
            let dur1 = start.elapsed();
            let ns1 = dur1.as_nanos() as f64 / ITERATIONS as f64;
            
            // Метод с умножением
            let start = Instant::now();
            for _ in 0..ITERATIONS {
                black_box(from_f32_mul(black_box(x)));
            }
            let dur2 = start.elapsed();
            let ns2 = dur2.as_nanos() as f64 / ITERATIONS as f64;
            
            results.push_str(&format!("  Bit shift:  {:>8?} total, {:>6.2} ns/op\n", dur1, ns1));
            results.push_str(&format!("  Mul:        {:>8?} total, {:>6.2} ns/op\n", dur2, ns2));
            results.push_str(&format!("  Speedup:    {:.2}x\n\n", ns2 / ns1));
        }
        results.push_str("\n================================\n");
        
        // Выводим всё сразу
        print!("{}", results);
        
        // Чтобы тест не проходил как успешный? Нет, пусть проходит
        assert!(true);
    }

    #[test]
    fn from_f64_test() {
        assert_eq!(UnitRanged::from_f64(0.0), UnitRanged::from_raw(0));
        assert_eq!(UnitRanged::from_f64(1.0), UnitRanged::from_raw(u32::MAX));
        assert_eq!(UnitRanged::from_f64(144.4), UnitRanged::from_raw(u32::MAX));
        assert_eq!(UnitRanged::from_f64(1e-45), UnitRanged::from_raw(0));
        assert_eq!(UnitRanged::from_f64(0.5), UnitRanged::from_raw(u32::MAX / 2 + 1));
        assert_eq!(UnitRanged::from_f64(f64::NAN), UnitRanged::MIN);
    }

    #[test]
    fn test_from_f64_speed() {
        fn from_f64_mul(x : f64) -> UnitRanged {
            const U32_MAX : f64 = u32::MAX as f64;
            let x = x.clamp(0.0, 1.0);
            return UnitRanged::from_raw((x * U32_MAX) as u32);
        }

        const ITERATIONS: u32 = 10_000_000;
        
        let mut results = String::new();
        results.push_str("\n=== F32 CONVERSION BENCHMARK ===\n\n");
        
        // Тестовые значения
        let values = [0.0f64, 0.1, 0.5, 0.999, 1.0, 1.3, 0e-45, -4.1];
        
        for &x in &values {
            results.push_str(&format!("Value: {}\n", x));
            results.push_str(&format!("{}\n", "-".repeat(50)));
            
            // Твой метод
            let start = Instant::now();
            for _ in 0..ITERATIONS {
                black_box(UnitRanged::from_f64(black_box(x)));
            }
            let dur1 = start.elapsed();
            let ns1 = dur1.as_nanos() as f64 / ITERATIONS as f64;
            
            // Метод с умножением
            let start = Instant::now();
            for _ in 0..ITERATIONS {
                black_box(from_f64_mul(black_box(x)));
            }
            let dur2 = start.elapsed();
            let ns2 = dur2.as_nanos() as f64 / ITERATIONS as f64;
            
            results.push_str(&format!("  Bit shift:  {:>8?} total, {:>6.2} ns/op\n", dur1, ns1));
            results.push_str(&format!("  Mul:        {:>8?} total, {:>6.2} ns/op\n", dur2, ns2));
            results.push_str(&format!("  Speedup:    {:.2}x\n\n", ns2 / ns1));
        }
        results.push_str("\n================================\n");
        
        // Выводим всё сразу
        print!("{}", results);
        
        // Чтобы тест не проходил как успешный? Нет, пусть проходит
        assert!(true);
    }

    #[test]
    fn to_f32_test() {
        assert_eq!(UnitRanged::MIN.to_f32(), 0.0);
        assert_eq!(UnitRanged::HALF.to_f32(), 0.5);
        assert_eq!(UnitRanged::from_f32(0.4).to_f32(), 0.4);
    }

    #[test]
    fn to_f64_test() {
        assert_eq!(UnitRanged::MIN.to_f64(), 0.0);
        assert_eq!(UnitRanged::HALF.to_f64(), 0.5);
        
        // Error is smaller than epsilon of `UnitRanged`
        assert_ne!(UnitRanged::from_f64(0.4).to_f64(), 0.4);

        assert!((UnitRanged::from_f64(0.4).to_f64() - 0.4).abs() < UnitRanged::F64_EPSILON);
    }

    #[test]
    fn mult_test() {
        let zero = UnitRanged::MIN;
        let one = UnitRanged::MAX;

        assert_eq!(zero * UnitRanged::from_f32(0.67), zero);
        assert_eq!(one * UnitRanged::from_f32(0.44), UnitRanged::from_f32(0.44));
        assert_eq!(one * one, one);
        assert_eq!(UnitRanged::from_f32(0.1) * UnitRanged::from_f32(0.2), UnitRanged::from_f32(0.02));
    }
}
