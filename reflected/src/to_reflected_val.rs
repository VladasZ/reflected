use std::str::FromStr;

use chrono::Duration;
use rust_decimal::Decimal;

pub trait ToReflectedVal<T> {
    fn to_reflected_val(&self) -> Result<T, String>;
}

macro_rules! impl_to_reflected_val {
    ($($t:ty),*) => {$(
        impl ToReflectedVal<$t> for &str {
            fn to_reflected_val(&self) -> Result<$t, String> {
                <$t>::from_str(self).map_err(|_e| format!("Failed to parse {}", self))
            }
        }
    )*};
}

impl_to_reflected_val!(
    i8, u8, i16, u16, i32, u32, i64, u64, f32, f64, isize, usize, String, Decimal
);

impl ToReflectedVal<Duration> for &str {
    fn to_reflected_val(&self) -> Result<Duration, String> {
        let seconds: i64 = self
            .parse()
            .map_err(|_e| format!("Failed to parse i64 for duration from: {self}"))?;

        Ok(Duration::new(seconds, 0).unwrap())
    }
}
