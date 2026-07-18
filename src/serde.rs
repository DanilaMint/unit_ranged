use crate::unit::UnitRanged;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

impl Serialize for UnitRanged {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f64(self.to_f64_fpu())
    }
}

impl<'de> Deserialize<'de> for UnitRanged {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_f64(UnitRangedVisitor)
    }
}

struct UnitRangedVisitor;

impl<'de> Visitor<'de> for UnitRangedVisitor {
    type Value = UnitRanged;

    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("a floating-point number in the range [0, 1]")
    }

    fn visit_f64<E: de::Error>(self, v: f64) -> Result<Self::Value, E> {
        if !(0.0..=1.0).contains(&v) {
            return Err(de::Error::invalid_value(de::Unexpected::Float(v), &self));
        }
        Ok(UnitRanged::from_f64_clamped_const(v))
    }

    fn visit_f32<E: de::Error>(self, v: f32) -> Result<Self::Value, E> {
        self.visit_f64(v as f64)
    }

    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
        if v > 1 {
            return Err(de::Error::invalid_value(de::Unexpected::Unsigned(v), &self));
        }
        Ok(UnitRanged::from_f64_clamped_const(v as f64))
    }

    fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
        if !(0..=1).contains(&v) {
            return Err(de::Error::invalid_value(de::Unexpected::Signed(v), &self));
        }
        Ok(UnitRanged::from_f64_clamped_const(v as f64))
    }
}
