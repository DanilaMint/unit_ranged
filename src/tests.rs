// Tests for UnitRanged type

#[cfg(test)]
use core::hint::black_box;
use crate::UnitRanged;
use num_traits::{ToPrimitive, FromPrimitive, CheckedAdd, CheckedSub, WrappingAdd, SaturatingAdd, SaturatingSub, Bounded, ToBytes, FromBytes};

// ============================================================================
// Conversion Tests
// ============================================================================

#[test]
fn test_from_f32_boundary_values() {
    // Minimum value
    assert_eq!(UnitRanged::from_f32_clamped(0.0), UnitRanged::MIN);

    // Maximum value
    assert_eq!(UnitRanged::from_f32_clamped(1.0), UnitRanged::MAX);

    // Values above 1.0 should clamp to MAX
    assert_eq!(UnitRanged::from_f32_clamped(1.5), UnitRanged::MAX);
    assert_eq!(UnitRanged::from_f32_clamped(144.4), UnitRanged::MAX);

    // Very small values should clamp to MIN
    assert_eq!(UnitRanged::from_f32_clamped(1e-45), UnitRanged::MIN);
    assert_eq!(UnitRanged::from_f32_clamped(0.0), UnitRanged::MIN);

    // Half value
    assert_eq!(UnitRanged::from_f32_clamped(0.5), UnitRanged::HALF);

    // NaN should clamp to MIN
    assert_eq!(UnitRanged::from_f32_clamped(f32::NAN), UnitRanged::MIN);

    // Negative values should clamp to MIN
    assert_eq!(UnitRanged::from_f32_clamped(-0.5), UnitRanged::MIN);
    assert_eq!(UnitRanged::from_f32_clamped(-1.0), UnitRanged::MIN);
    assert_eq!(UnitRanged::from_f32_clamped(-100.0), UnitRanged::MIN);
}

#[test]
fn test_from_f32_precision() {
    // Test various precision values
    let quarter = UnitRanged::from_f32_clamped(0.25);
    let three_quarters = UnitRanged::from_f32_clamped(0.75);

    assert!(quarter.into_raw() < UnitRanged::HALF.into_raw());
    assert!(three_quarters.into_raw() > UnitRanged::HALF.into_raw());

    // Test round-trip conversion
    let original = 0.4f32;
    let converted: f32 = UnitRanged::from_f32_clamped(original).into();
    assert!((converted - original).abs() < f32::EPSILON * 10.0);
}

#[test]
fn test_from_f64_boundary_values() {
    // Minimum value
    assert_eq!(UnitRanged::from_f64_clamped(0.0), UnitRanged::MIN);

    // Maximum value
    assert_eq!(UnitRanged::from_f64_clamped(1.0), UnitRanged::MAX);

    // Values above 1.0 should clamp to MAX
    assert_eq!(UnitRanged::from_f64_clamped(1.5), UnitRanged::MAX);
    assert_eq!(UnitRanged::from_f64_clamped(144.4), UnitRanged::MAX);

    // Very small values should clamp to MIN
    assert_eq!(UnitRanged::from_f64_clamped(1e-45), UnitRanged::MIN);
    assert_eq!(UnitRanged::from_f64_clamped(0.0), UnitRanged::MIN);

    // Half value
    assert_eq!(UnitRanged::from_f64_clamped(0.5), UnitRanged::HALF);

    // NaN should clamp to MIN
    assert_eq!(UnitRanged::from_f64_clamped(f64::NAN), UnitRanged::MIN);

    // Negative values should clamp to MIN
    assert_eq!(UnitRanged::from_f64_clamped(-0.5), UnitRanged::MIN);
    assert_eq!(UnitRanged::from_f64_clamped(-1.0), UnitRanged::MIN);
    assert_eq!(UnitRanged::from_f64_clamped(-100.0), UnitRanged::MIN);
}

#[test]
fn test_to_f32_conversion() {
    assert_eq!(UnitRanged::MIN.to_f32(), Some(0.0));
    assert_eq!(UnitRanged::HALF.to_f32(), Some(0.5));

    // Test round-trip
    let original = 0.4f32;
    let ur = UnitRanged::from_f32_clamped(original);
    let converted: f32 = ur.into();
    assert!((converted - original).abs() < f32::EPSILON * 10.0);
}

#[test]
fn test_to_f64_conversion() {
    assert_eq!(UnitRanged::MIN.to_f64(), Some(0.0));
    assert_eq!(UnitRanged::HALF.to_f64(), Some(0.5));

    // Error is smaller than epsilon of UnitRanged
    let ur = UnitRanged::from_f64_clamped(0.4);
    let converted = ur.to_f64().unwrap();
    assert_ne!(converted, 0.4);
    assert!((converted - 0.4).abs() < UnitRanged::F64_EPSILON);
}

// ============================================================================
// Arithmetic Operator Tests
// ============================================================================

#[test]
fn test_addition_operator() {
    let a = UnitRanged::from_f32_clamped(0.25);
    let b = UnitRanged::from_f32_clamped(0.25);
    let result = a + b;

    // 0.25 + 0.25 = 0.5
    assert_eq!(result, UnitRanged::HALF);
}

#[test]
fn test_subtraction_operator() {
    let a = UnitRanged::from_f32_clamped(0.75);
    let b = UnitRanged::from_f32_clamped(0.25);
    let result = a - b;

    // 0.75 - 0.25 = 0.5
    assert_eq!(result, UnitRanged::HALF);
}

#[test]
fn test_multiplication_operator() {
    let zero = UnitRanged::MIN;
    let one = UnitRanged::MAX;

    // Zero * anything = zero
    assert_eq!(zero * UnitRanged::from_f32_clamped(0.67), zero);

    // One * anything = anything (with possible precision loss)
    let result = one * UnitRanged::from_f32_clamped(0.44);
    let expected = UnitRanged::from_f32_clamped(0.44);
    assert!((result.into_raw() as i64 - expected.into_raw() as i64).abs() < 2);

    // One * one = one (may have precision issues)
    let result = one * one;
    assert!(result.into_raw() >= UnitRanged::MAX.into_raw() - 1);

    // 0.1 * 0.2 = 0.02 (with some precision loss)
    let result = UnitRanged::from_f32_clamped(0.1) * UnitRanged::from_f32_clamped(0.2);
    let result_f64: f64 = result.into();
    assert!((result_f64 - 0.02).abs() < UnitRanged::F64_EPSILON * 1000.0);
}

#[test]
fn test_division_operator() {
    // Test basic division works without crashing
    let eighth = UnitRanged::from_f32_clamped(0.125);
    let quarter = UnitRanged::from_f32_clamped(0.25);
    let half = UnitRanged::HALF;

    // These should work without overflow
    let _result1 = eighth / half;
    let _result2 = quarter / half;
    let _result3 = eighth / quarter;

    // Test that division by a smaller number might overflow but doesn't crash
    let _result4 = half / eighth; // This might overflow, that's ok
}

#[test]
fn test_remainder_operator() {
    // Test basic remainder works without crashing
    let eighth = UnitRanged::from_f32_clamped(0.125);
    let quarter = UnitRanged::from_f32_clamped(0.25);
    let half = UnitRanged::HALF;

    // These should work
    let _result1 = half % quarter;
    let _result2 = quarter % eighth;

    // Test that remainder operations produce valid UnitRanged values
    let result = half % quarter;
    assert!(result.into_raw() <= UnitRanged::MAX.into_raw());
}

// ============================================================================
// Comparison Tests
// ============================================================================

#[test]
fn test_ordering() {
    let small = UnitRanged::from_f32_clamped(0.25);
    let medium = UnitRanged::HALF;
    let large = UnitRanged::from_f32_clamped(0.75);

    assert!(small < medium);
    assert!(medium < large);
    assert!(small < large);

    assert_eq!(small.cmp(&medium), std::cmp::Ordering::Less);
    assert_eq!(medium.cmp(&medium), std::cmp::Ordering::Equal);
    assert_eq!(large.cmp(&medium), std::cmp::Ordering::Greater);
}

#[test]
fn test_min_max() {
    let a = UnitRanged::from_f32_clamped(0.25);
    let b = UnitRanged::from_f32_clamped(0.75);

    assert_eq!(a.min(b), a);
    assert_eq!(a.max(b), b);
}

#[test]
fn test_clamp() {
    let low = UnitRanged::from_f32_clamped(0.25);
    let high = UnitRanged::from_f32_clamped(0.75);

    assert_eq!(UnitRanged::MIN.clamp(low, high), low);
    assert_eq!(UnitRanged::HALF.clamp(low, high), UnitRanged::HALF);
    assert_eq!(UnitRanged::MAX.clamp(low, high), high);
}

// ============================================================================
// Num Traits Tests
// ============================================================================

#[test]
fn test_checked_operations() {
    let small = UnitRanged::from_f32_clamped(0.25);
    let large = UnitRanged::from_f32_clamped(0.75);

    // Checked add should succeed
    assert!(small.checked_add(&small).is_some());

    // Checked add with overflow should fail
    let max_val = UnitRanged::MAX;
    assert!(max_val.checked_add(&max_val).is_none());

    // Checked sub should succeed
    assert!(large.checked_sub(&small).is_some());

    // Checked sub with underflow should fail
    assert!(small.checked_sub(&large).is_none());
}

#[test]
fn test_wrapping_operations() {
    let max = UnitRanged::MAX;

    // Wrapping add should overflow and wrap around
    let wrapped = max.wrapping_add(&max);
    assert!(wrapped.into_raw() < max.into_raw());
}

#[test]
fn test_saturating_operations() {
    let max = UnitRanged::MAX;
    let min = UnitRanged::MIN;
    let half = UnitRanged::HALF;

    // Saturating add should saturate at MAX
    assert_eq!(max.saturating_add(&max), UnitRanged::MAX);
    assert_eq!(half.saturating_add(&half), UnitRanged::MAX);

    // Saturating sub should saturate at MIN
    assert_eq!(min.saturating_sub(&half), UnitRanged::MIN);
}

#[test]
fn test_bounded() {
    assert_eq!(UnitRanged::min_value(), UnitRanged::MIN);
    assert_eq!(UnitRanged::max_value(), UnitRanged::MAX);
}

#[test]
fn test_from_primitive() {
    // Test integer conversions
    assert_eq!(UnitRanged::from_u8(0), Some(UnitRanged::MIN));
    assert_eq!(UnitRanged::from_u8(1), Some(UnitRanged::MAX));
    assert_eq!(UnitRanged::from_u8(2), None);

    assert_eq!(UnitRanged::from_i32(0), Some(UnitRanged::MIN));
    assert_eq!(UnitRanged::from_i32(1), Some(UnitRanged::MAX));
    assert_eq!(UnitRanged::from_i32(-1), None);

    // Float conversions
    assert!(UnitRanged::from_f32(0.5).is_some());
    assert!(UnitRanged::from_f32(-1.0).is_none());
    assert!(UnitRanged::from_f32(2.0).is_none());
    assert!(UnitRanged::from_f32(f32::NAN).is_none());
}

#[test]
fn test_to_primitive() {
    let min = UnitRanged::MIN;
    let max = UnitRanged::MAX;

    // MIN converts to 0, MAX converts to 1
    assert_eq!(min.to_u8(), Some(0));
    assert_eq!(max.to_u8(), Some(1));

    assert_eq!(min.to_i32(), Some(0));
    assert_eq!(max.to_i32(), Some(1));

    // Float conversions (may have precision issues with MAX)
    assert_eq!(min.to_f32(), Some(0.0));
    let max_f32 = max.to_f32().unwrap();
    assert!((max_f32 - 1.0).abs() < f32::EPSILON);
}

#[test]
fn test_byte_conversions() {
    let original = UnitRanged::from_f32_clamped(0.5);

    // Test little-endian
    let bytes_le = original.to_le_bytes();
    let restored_le = UnitRanged::from_le_bytes(&bytes_le);
    assert_eq!(original, restored_le);

    // Test big-endian
    let bytes_be = original.to_be_bytes();
    let restored_be = UnitRanged::from_be_bytes(&bytes_be);
    assert_eq!(original, restored_be);

    // Test native-endian
    let bytes_ne = original.to_ne_bytes();
    let restored_ne = UnitRanged::from_ne_bytes(&bytes_ne);
    assert_eq!(original, restored_ne);
}

// ============================================================================
// Performance Tests (Benchmarks)
// ============================================================================

#[test]
fn benchmark_conversion_f32() {
    const ITERATIONS: u32 = 1_000_000;
    let test_values = [0.0f32, 0.1, 0.5, 0.999, 1.0];

    for &value in &test_values {
        for _ in 0..ITERATIONS {
            black_box(UnitRanged::from_f32_clamped(black_box(value)));
        }
    }
}

#[test]
fn benchmark_conversion_f64() {
    const ITERATIONS: u32 = 1_000_000;
    let test_values = [0.0f64, 0.1, 0.5, 0.999, 1.0];

    for &value in &test_values {
        for _ in 0..ITERATIONS {
            black_box(UnitRanged::from_f64_clamped(black_box(value)));
        }
    }
}

#[test]
fn benchmark_arithmetic_operations() {
    const ITERATIONS: u32 = 1_000_000;
    let a = UnitRanged::from_f32_clamped(0.3);
    let b = UnitRanged::from_f32_clamped(0.2);

    for _ in 0..ITERATIONS {
        black_box(black_box(a) + black_box(b));
        // Skip subtraction to avoid underflow in benchmark
        black_box(black_box(a) * black_box(b));
        black_box(black_box(a) / black_box(b));
    }
}

// ============================================================================
// Edge Cases and Special Values
// ============================================================================

#[test]
fn test_nan_handling() {
    let nan_f32 = f32::NAN;
    let nan_f64 = f64::NAN;

    // All NaN conversions should result in MIN
    assert_eq!(UnitRanged::from_f32_clamped(nan_f32), UnitRanged::MIN);
    assert_eq!(UnitRanged::from_f64_clamped(nan_f64), UnitRanged::MIN);
}

#[test]
fn test_infinity_handling() {
    let pos_inf_f32 = f32::INFINITY;
    let pos_inf_f64 = f64::INFINITY;
    let neg_inf_f32 = f32::NEG_INFINITY;
    let neg_inf_f64 = f64::NEG_INFINITY;

    // Positive infinity should clamp to MAX
    assert_eq!(UnitRanged::from_f32_clamped(pos_inf_f32), UnitRanged::MAX);
    assert_eq!(UnitRanged::from_f64_clamped(pos_inf_f64), UnitRanged::MAX);

    // Negative infinity should clamp to MIN
    assert_eq!(UnitRanged::from_f32_clamped(neg_inf_f32), UnitRanged::MIN);
    assert_eq!(UnitRanged::from_f64_clamped(neg_inf_f64), UnitRanged::MIN);
}

#[test]
fn test_denormalized_numbers() {
    // Test very small positive denormalized numbers
    let tiny_f32 = f32::from_bits(0x00000001); // Smallest positive f32
    assert_eq!(UnitRanged::from_f32_clamped(tiny_f32), UnitRanged::MIN);

    let tiny_f64 = f64::from_bits(0x0000000000000001); // Smallest positive f64
    assert_eq!(UnitRanged::from_f64_clamped(tiny_f64), UnitRanged::MIN);
}

#[test]
fn test_round_trip_conversions() {
    let test_values = [0.0f64, 0.001, 0.1, 0.25, 0.5, 0.75, 0.999, 1.0];

    for &value in &test_values {
        let ur = UnitRanged::from_f64_clamped(value);
        let converted: f64 = ur.into();
        let error = (converted - value).abs();
        assert!(error < UnitRanged::F64_EPSILON * 1000.0,
               "Round-trip error too large for {}: {}", value, error);
    }
}
