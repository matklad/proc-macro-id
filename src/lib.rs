extern crate proc_macro;

#[proc_macro]
pub fn id(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    eprintln!("id!({})", input);
    input
}

#[proc_macro_attribute]
pub fn id_attr(attr: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    eprintln!("id_attr!({}, {})", attr, input);
    input
}
