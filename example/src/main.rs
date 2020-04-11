use encrypted_id::prelude::*;
#[macro_use]
extern crate encrypted_id_derive;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Default, Encrypt, Decrypt)] // Encrypt, Decrypt
#[encdec_opts(opts(sub_key = "enky_demo_sub_key"))]
pub struct Demo {
    pub id: u64,
    pub name: String,
}

//fn enc_test() {
//    init_conf("se(vh!38e21qca#9m7g0#7tyq+a*z#imfjr10&iezsfmh6l)v(");
//    let e = Demo {
//        id: 5,
//        name: "foo".to_string(),
//    };
//    let ekey = e.ekey().unwrap();
//    let dkey = e.dkey(&ekey).unwrap();
//    assert_eq!("mZZLspleIzJqmKLa2Oio_g", ekey);
//    assert_eq!(5, dkey);
//}
