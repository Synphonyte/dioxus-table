mod table;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_derive(TableData, attributes(table))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let ast = &input;
    let table_data = table::TableDataDeriveInput::from_derive_input(&ast).expect("Wrong options");
    let data = quote!(#table_data);
    data.into()
}
