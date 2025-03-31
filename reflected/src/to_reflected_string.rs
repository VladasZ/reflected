use chrono::{Duration, NaiveDateTime};
use rust_decimal::{Decimal, prelude::Zero};

pub trait ToReflectedString {
    fn to_reflected_string(&self) -> String;
}

macro_rules! impl_to_string_optional {
    ($($t:ty),*) => {$(
        impl ToReflectedString for Option<$t> {
            fn to_reflected_string(&self) -> String {
                self.clone().map(|a| a.to_string()).unwrap_or("NULL".to_string())
            }
        }
    )*};
}

impl_to_string_optional!(
    i8,
    u8,
    i16,
    u16,
    i32,
    u32,
    i64,
    u64,
    isize,
    usize,
    &str,
    String,
    Decimal,
    NaiveDateTime
);

macro_rules! impl_custom_to_string_optional {
    ($($t:ty),*) => {$(
        impl ToReflectedString for Option<$t> {
            fn to_reflected_string(&self) -> String {
                self.clone().map(|a| a.to_reflected_string()).unwrap_or("NULL".to_string())
            }
        }
    )*};
}

impl ToReflectedString for f64 {
    fn to_reflected_string(&self) -> String {
        if self.fract().is_zero() {
            format!("{self}.0")
        } else {
            self.to_string()
        }
    }
}

impl ToReflectedString for f32 {
    fn to_reflected_string(&self) -> String {
        if self.fract().is_zero() {
            format!("{self}.0")
        } else {
            self.to_string()
        }
    }
}

impl ToReflectedString for Duration {
    fn to_reflected_string(&self) -> String {
        self.num_seconds().to_string()
    }
}

impl_custom_to_string_optional!(f64, f32, Duration);
