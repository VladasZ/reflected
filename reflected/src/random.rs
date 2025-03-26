use chrono::Utc;
use fake::{Fake, Faker};
use rust_decimal::Decimal;

use crate::Type;

pub(crate) fn random_val(tp: Type) -> Option<String> {
    match tp {
        Type::Text => 8.fake::<String>().into(),
        Type::Integer | Type::Float => (0..1_000).fake::<i64>().to_string().into(),
        Type::Date => Utc::now().naive_utc().to_string().into(),
        Type::Decimal => Decimal::new(i64::from((u32::MIN..u32::MAX).fake::<u32>()), (1..6).fake())
            .to_string()
            .into(),
        Type::Bool => (0..2).fake::<i32>().to_string().into(),
        Type::Optional(opt) => {
            if Faker.fake::<bool>() {
                random_val(opt.to_type())
            } else {
                None
            }
        }
        Type::Custom => unreachable!("Failed to gen random value for: {tp:?}"),
    }
}
