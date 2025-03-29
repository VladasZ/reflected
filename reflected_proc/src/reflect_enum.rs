use proc_macro::TokenStream;
use syn::DataEnum;

pub fn reflect_enum(en: &mut DataEnum) -> TokenStream {
    dbg!(en);

    unimplemented!("Support of enum reflection is not implemented yet")
}
