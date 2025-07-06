use crate::{Field, random::random_val};

pub trait Reflected: Send + Default + 'static {
    fn type_name() -> &'static str;

    fn fields() -> &'static [Field<Self>];

    fn get_value(&self, field: Field<Self>) -> String;
    fn set_value(&mut self, field: Field<Self>, value: Option<&str>);

    #[cfg(feature = "sqlx_bind")]
    fn bind_to_sqlx_query<'q, O>(
        self,
        query: sqlx::query::QueryAs<'q, sqlx::Postgres, O, <sqlx::Postgres as sqlx::Database>::Arguments<'q>>,
    ) -> sqlx::query::QueryAs<'q, sqlx::Postgres, O, <sqlx::Postgres as sqlx::Database>::Arguments<'q>>;

    fn field_by_name(name: &str) -> Field<Self> {
        *Self::fields().iter().find(|a| a.name == name).unwrap_or_else(|| {
            panic!(
                "Failed to get field_by_name of {}. Field name: {name}",
                Self::type_name(),
            )
        })
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
