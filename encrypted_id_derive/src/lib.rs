#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;
extern crate darling;

use crate::darling::FromDeriveInput;
use crate::darling::FromMeta;

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

#[proc_macro_derive(Encrypt, attributes(encdec_opts))]
pub fn encryption(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input: syn::DeriveInput =
        parse_macro_input!(input as syn::DeriveInput);
    let attrs: EncDecOpts = match EncDecOpts::from_derive_input(&derive_input) {
        Ok(attrs) => attrs,
        Err(err) => {
            return err.write_errors().into();
        }
    };

    let sub_key: String = attrs.opts.sub_key;
    let ident: syn::Ident = derive_input.ident;
    let generics: syn::Generics = derive_input.generics;

    quote!(
        impl Encrypt for #ident #generics {
            fn ekey(&self) -> EResult<String> {
                encrypt::encode(self.id as u64, #sub_key)
            }
        }
    )
    .into()
}

#[proc_macro_derive(Decrypt, attributes(encdec_opts))]
pub fn decryption(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input: syn::DeriveInput =
        parse_macro_input!(input as syn::DeriveInput);
    let attrs: EncDecOpts = match EncDecOpts::from_derive_input(&derive_input) {
        Ok(attrs) => attrs,
        Err(err) => {
            return err.write_errors().into();
        }
    };

    let sub_key = attrs.opts.sub_key;
    let generics = derive_input.generics;
    let ident: syn::Ident = derive_input.ident;
    quote!(
        impl Decrypt for #ident #generics {
            fn id(&self, ekey: &str) -> EResult<u64> {
                decrypt::decode(ekey, #sub_key)
            }
        }
    )
    .into()
}
