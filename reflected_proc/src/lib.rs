use std::str::FromStr;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Fields, FieldsNamed, GenericArgument, Ident, Meta,
    NestedMeta, PathArguments, Type,
    __private::{Span, TokenStream2},
};

use crate::field::Field;

mod field;

/// Data must also derive `Default`
#[proc_macro_derive(Reflected)]
pub fn reflected(stream: TokenStream) -> TokenStream {
    let mut stream = parse_macro_input!(stream as DeriveInput);

    let Data::Struct(data) = &mut stream.data else {
        panic!("`db_entity` macro has to be used with structs")
    };

    let Fields::Named(struct_fields) = &mut data.fields else {
        panic!()
    };

    let (rename, fields) = parse_fields(struct_fields);

    let name = stream.ident.clone();

    let name_string = if let Some(rename) = rename {
        TokenStream2::from_str(&format!("\"{rename}\""))
    } else {
        TokenStream2::from_str(&format!("\"{name}\""))
    }
    .unwrap();

    let fields_struct_name = Ident::new(&format!("{name}Fields"), Span::call_site());

    let fields_struct = fields_struct(&name, &fields);
    let fields_const_var = fields_const_var(&name, &fields);
    let fields_reflect = fields_reflect(&name, &fields);
    let simple_fields_reflect = simple_fields_reflect(&name, &fields);
    let get_value = fields_get_value(&fields);
    let set_value = fields_set_value(&fields);

    quote! {
        #[derive(Debug)]
        pub struct #fields_struct_name {
            #fields_struct
        }

        impl #name {
            pub const FIELDS: #fields_struct_name = #fields_struct_name {
                #fields_const_var
            };
        }

        impl reflected::Reflected for #name {
            fn type_name() -> &'static str {
                #name_string
            }

            fn fields() -> &'static [reflected::Field<Self>] {
                &[
                    #fields_reflect
                ]
            }

            fn simple_fields() -> &'static [reflected::Field<Self>] {
                &[
                    #simple_fields_reflect
                ]
            }

            fn get_value(&self, field: reflected::Field<Self>) -> String {
                use std::borrow::Borrow;
                use reflected::ToReflectedString;
                let field = field.borrow();

                if field.is_custom() {
                    panic!("get_value method is not supported for custom types: {field:?}");
                }

                match field.name {
                    #get_value
                    _ => unreachable!("Invalid field name in get_value: {}", field.name),
                }
            }

            fn set_value(&mut self, field: reflected::Field<Self>, value: Option<&str>) {
                use reflected::ToReflectedVal;
                use std::borrow::Borrow;
                let field = field.borrow();
                match field.name {
                    #set_value
                    _ => unreachable!("Invalid field name in set_value"),
                }
            }
        }
    }
    .into()
}

fn fields_const_var(type_name: &Ident, fields: &Vec<Field>) -> TokenStream2 {
    let mut res = quote!();

    let type_name = TokenStream2::from_str(&format!("\"{type_name}\"")).unwrap();

    for field in fields {
        let name = &field.name;

        let field_type = field.field_type();

        let field_type_name = field.type_as_string();
        let name_string = field.name_as_string();

        let optional = field.optional;

        let tp = if optional {
            quote! {
                tp: reflected::Type::#field_type.to_optional()
            }
        } else {
            quote! {
                tp: reflected::Type::#field_type
            }
        };

        res = quote! {
            #res
            #name: reflected::Field {
                name: #name_string,
                #tp,
                type_name: #field_type_name,
                parent_name: #type_name,
                optional: #optional,
                _p: std::marker::PhantomData,
            },
        }
    }

    res
}

fn fields_struct(type_name: &Ident, fields: &Vec<Field>) -> TokenStream2 {
    let mut res = quote!();

    for field in fields {
        let name = &field.name;
        res = quote! {
            #res
            pub #name: reflected::Field<#type_name>,
        }
    }

    quote! {
        #res
    }
}

fn fields_reflect(name: &Ident, fields: &Vec<Field>) -> TokenStream2 {
    let mut res = quote!();

    for field in fields {
        let field_name = &field.name;
        res = quote! {
            #res
            #name::FIELDS.#field_name,
        }
    }

    res
}

fn simple_fields_reflect(name: &Ident, fields: &Vec<Field>) -> TokenStream2 {
    let mut res = quote!();

    for field in fields {
        if !field.is_simple() {
            continue;
        }
        let field_name = &field.name;
        res = quote! {
            #res
            #name::FIELDS.#field_name,
        }
    }

    res
}

fn fields_get_value(fields: &Vec<Field>) -> TokenStream2 {
    let mut res = quote!();

    for field in fields {
        if field.custom() {
            continue;
        }

        let field_name = &field.name;
        let name_string = field.name_as_string();

        if field.is_bool() {
            if field.optional {
                res = quote! {
                    #res
                    #name_string => self.#field_name.map(|a| if a { "1" } else { "0" }.to_string()).unwrap_or("NULL".to_string()),
                }
            } else {
                res = quote! {
                    #res
                    #name_string => if self.#field_name { "1" } else { "0" }.to_string(),
                }
            }
        } else if field.optional || field.is_float() {
            res = quote! {
                #res
                #name_string => self.#field_name.to_reflected_string(),
            }
        } else {
            res = quote! {
                #res
                #name_string => self.#field_name.to_string(),
            }
        }
    }

    res
}

fn fields_set_value(fields: &Vec<Field>) -> TokenStream2 {
    let mut res = quote!();

    for field in fields {
        if field.custom() {
            continue;
        }

        let field_name = &field.name;
        let name_string = field.name_as_string();

        if field.is_bool() {
            if field.optional {
                res = quote! {
                    #res
                    #name_string =>  {
                        self.#field_name = value.map(|a| match a {
                            "0" => false,
                            "1" => true,
                            _ => unreachable!("Invalid value in bool: {value:?}")
                        })
                    },
                }
            } else {
                res = quote! {
                    #res
                    #name_string =>  {
                        self.#field_name = match value.unwrap() {
                            "0" => false,
                            "1" => true,
                            _ => unreachable!("Invalid value in bool: {value:?}")
                        }
                    },
                }
            }
        } else if field.is_date() {
            res = quote! {
                #res
                #name_string => self.#field_name = chrono::NaiveDateTime::parse_from_str(&value.unwrap(), "%Y-%m-%d %H:%M:%S%.9f").unwrap(),
            }
        } else if field.optional {
            res = quote! {
                #res
                #name_string => self.#field_name = value.map(|a| a.to_reflected_val()
                    .expect(&format!("Failed to convert to: {} from: {}", #name_string, a))),
            }
        } else {
            res = quote! {
                #res
                #name_string => self.#field_name = value.unwrap().to_reflected_val()
                .expect(&format!("Failed to convert to: {} from: {}", #name_string, value.unwrap())),
            }
        }
    }

    res
}

fn parse_fields(fields: &FieldsNamed) -> (Option<String>, Vec<Field>) {
    let mut rename: Option<String> = None;

    let fields: Vec<Field> = fields
        .named
        .iter()
        .map(|field| {
            let name = field.ident.as_ref().unwrap().clone();
            let mut optional = false;

            let Type::Path(path) = &field.ty else {
                unreachable!("invalid parse_fields")
            };

            let mut tp = path.path.segments.first().unwrap().ident.clone();

            if tp == "Option" {
                optional = true;
                let args = &path.path.segments.first().unwrap().arguments;
                if let PathArguments::AngleBracketed(args) = args {
                    if let GenericArgument::Type(generic_tp) = args.args.first().unwrap() {
                        let ident = generic_tp.to_token_stream().to_string();
                        let ident = Ident::new(&ident, Span::call_site());
                        tp = ident;
                    } else {
                        unreachable!()
                    }
                } else {
                    unreachable!()
                }
            }

            let _attrs: Vec<String> = field
                .attrs
                .iter()
                .map(|a| {
                    let name = get_attribute_name(a);
                    if name == "name" {
                        rename = get_attribute_value(a).expect("name attribute should have value").into();
                    }
                    name
                })
                .collect();

            Field { name, tp, optional }
        })
        .collect();

    (rename, fields)
}

fn get_attribute_name(attribute: &Attribute) -> String {
    attribute.path.segments.first().unwrap().ident.to_string()
}

fn get_attribute_value(attribute: &Attribute) -> Option<String> {
    if let Ok(Meta::List(meta_list)) = attribute.parse_meta() {
        if let NestedMeta::Meta(Meta::Path(path)) = &meta_list.nested[0] {
            return Some(path.segments.last()?.ident.to_string());
        }
    }
    None
}
