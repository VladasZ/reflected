use sqlx::{Database, Postgres, query::QueryAs};

use crate::{Field, random::random_val};

pub trait Reflected: Send + Default + 'static {
    fn type_name() -> &'static str;

    fn fields() -> &'static [Field<Self>];

    fn get_value(&self, field: Field<Self>) -> String;
    fn set_value(&mut self, field: Field<Self>, value: Option<&str>);

    fn bind_to_sqlx_query<'q, O>(
        self,
        query: QueryAs<'q, Postgres, O, <Postgres as Database>::Arguments<'q>>,
    ) -> QueryAs<'q, Postgres, O, <Postgres as Database>::Arguments<'q>>;

    fn field_by_name(name: &str) -> Field<Self> {
        *Self::fields().iter().find(|a| a.name == name).unwrap()
    }

    fn value_by_name(&self, name: &str) -> String {
        self.get_value(Self::field_by_name(name))
    }

    fn random() -> Self {
        let mut res = Self::default();

        for field in Self::fields() {
            if field.is_enum() {
                continue;
            }
            res.set_value(*field, random_val(field).as_deref());
        }

        res
    }
}
