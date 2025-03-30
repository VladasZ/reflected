use std::str::FromStr;

use syn::{__private::Span, Ident};

use crate::TokenStream2;

#[derive(Debug)]
pub(crate) struct Field {
    pub(crate) name:     Ident,
    pub(crate) tp:       Ident,
    pub(crate) optional: bool,
}

impl Field {
    pub(crate) fn type_as_string(&self) -> TokenStream2 {
        TokenStream2::from_str(&format!("\"{}\"", self.tp)).unwrap()
    }

    pub(crate) fn name_as_string(&self) -> TokenStream2 {
        TokenStream2::from_str(&format!("\"{}\"", self.name)).unwrap()
    }

    pub(crate) fn _id(&self) -> bool {
        self.name == Ident::new("id", Span::call_site())
    }

    pub(crate) fn _is_foreign_id(&self) -> bool {
        self.name.to_string().contains("_id")
    }

    pub(crate) fn custom(&self) -> bool {
        self.field_type() == Ident::new("Custom", Span::call_site())
    }

    pub(crate) fn is_bool(&self) -> bool {
        self.field_type() == "Bool"
    }

    pub(crate) fn is_float(&self) -> bool {
        self.field_type() == "Float"
    }

    pub(crate) fn is_duration(&self) -> bool {
        self.field_type() == "Duration"
    }

    pub(crate) fn is_date(&self) -> bool {
        self.field_type() == "Date"
    }

    pub(crate) fn _is_enum(&self) -> bool {
        self.field_type() == "Enum"
    }

    pub(crate) fn field_type(&self) -> Ident {
        let float = Ident::new("Float", Span::call_site());
        let integer = Ident::new("Integer", Span::call_site());
        let text = Ident::new("Text", Span::call_site());
        let en = Ident::new("Enum", Span::call_site());
        let date = Ident::new("Date", Span::call_site());
        let decimal = Ident::new("Decimal", Span::call_site());
        let bool = Ident::new("Bool", Span::call_site());
        let duration = Ident::new("Duration", Span::call_site());

        match self.tp.to_string().as_str() {
            "f32" | "f64" => float,
            "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "isize" | "usize" => integer,
            "String" => text,
            "NaiveDateTime" | "DateTime" => date,
            "Decimal" => decimal,
            "bool" => bool,
            "Duration" => duration,
            _ => en,
        }
    }
}
