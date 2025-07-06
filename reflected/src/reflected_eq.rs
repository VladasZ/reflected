use crate::Reflected;

pub trait ReflectedEq {
    fn assert_eq(&self, other: &Self);
}

impl<T: Reflected> ReflectedEq for T {
    fn assert_eq(&self, other: &Self) {
        for field in T::fields() {
            let a = self.get_value(*field);
            let b = other.get_value(*field);

            if field.is_float() || field.is_decimal() {
                let a: f64 = a.parse().unwrap_or_else(|err| panic!("Failed to parse f64 from {a}: {err}"));
                let b: f64 = b.parse().unwrap_or_else(|err| panic!("Failed to parse f64 from {a}: {err}"));

                let diff = (a - b).abs();

                assert!(
                    diff <= 0.001,
                    "Reflected eq error: Field: {field:?}\nLeft: {a}, Right: {b}"
                );

                continue;
            }

            assert_eq!(
                a, b,
                "Reflected eq error: Field: {field:?}\nLeft: {a}, Right: {b}"
            );
        }
    }
}
