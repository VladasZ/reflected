use chrono::Utc;
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng, Rng,
};
use rust_decimal::Decimal;

use crate::Type;

pub(crate) fn random_val(tp: Type) -> Option<String> {
    let mut rng = thread_rng();

    match tp {
        Type::Text => Alphanumeric.sample_string(&mut rng, 8).into(),
        Type::Integer | Type::Float => rng.gen_range(0..1_000_000_000).to_string().into(),
        Type::Date => Utc::now().naive_utc().to_string().into(),
        Type::Decimal => Decimal::new(rng.gen_range(u32::MIN..u32::MAX).into(), rng.gen_range(1..6))
            .to_string()
            .into(),
        Type::Bool => rng.gen_range(0..2).to_string().into(),
        Type::Optional(opt) => {
            if rng.gen() {
                random_val(opt.to_type())
            } else {
                None
            }
        }
        Type::Custom => unreachable!("Failed to gen random value for: {tp:?}"),
    }
}
