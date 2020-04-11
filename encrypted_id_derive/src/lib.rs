#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;
extern crate darling;

use crate::darling::FromMeta;
use crate::darling::FromDeriveInput;

#[derive(Default, FromMeta, Debug)]
#[darling(default)]
struct EncDecArgs {
    sub_key: String,
}

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(encdec_opts), forward_attrs(allow, doc, cfg))]
struct EncDecOpts {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    opts: EncDecArgs,
}

#[proc_macro_derive(Encrypted, attributes(encdec_opts))]
pub fn encryption(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);
    let attrs = match EncDecOpts::from_derive_input(&derive_input) {
        Ok(attrs) => attrs,
        Err(err) => {
            return err.write_errors().into();
        }
    };
    quote!().into()
}
