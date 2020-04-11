#[macro_use]
extern crate encrypted_id_derive;
use encrypted_id::prelude::*;

#[derive(Debug, Default, Encrypt, Decrypt)] // Encrypt, Decrypt
#[encdec_opts(opts(sub_key = "enky_demo_sub_key"))]
pub struct Demo {
    pub id: u64,
    pub name: String,
}

fn enc_test() {
    let e = Demo {
        id: 5,
        name: "foo".to_string(),
    };
    let ekey = e.ekey().unwrap();
    let id = e.id(&ekey).unwrap();
    assert_eq!("AuovBQ1f2B1AmEd3o0Uq1Q", ekey);
    assert_eq!(5, id);
}


fn main(){
    init_conf("df(vh!3*8e21@qca#3)w#7ta*z#!bhsde43&#iez3sf5m1#h6l");
    enc_test()
}