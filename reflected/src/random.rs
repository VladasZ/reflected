use chrono::Utc;
use fake::{Fake, Faker, faker::internet::en::FreeEmail};
use rust_decimal::Decimal;

use crate::{Field, Type};

pub(crate) fn random_val<T>(field: &Field<T>) -> Option<String> {
    match field.tp {
        Type::Text => {
            if field.name.contains("email") {
                FreeEmail().fake::<String>().into()
            } else {
                16.fake::<String>().into()
            }
        }
        Type::Integer | Type::Float => (0..1_000).fake::<i64>().to_string().into(),
        Type::Date => Utc::now().naive_utc().to_string().into(),
        Type::Decimal => Decimal::new(i64::from((u32::MIN..u32::MAX).fake::<u32>()), (1..6).fake())
            .to_string()
            .into(),
        Type::Bool => (0..2).fake::<i32>().to_string().into(),
        Type::Optional(_) => {
            if Faker.fake::<bool>() {
                random_val(&field.non_optional())
            } else {
                None
            }
        }
        Type::Enum => unreachable!("Failed to gen random value for: {field:?}"),
    }
}
