# Rust Encryption and Decryption

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/encrypted_id)](https://crates.io/crates/encrypted_id)


#### Usage

```toml
[dependencies]
encrypted_id = "0.1.4"
encrypted_id_derive = "0.1.0"
```

* Encryption and Decryption in action
* To make any struct encrypt-able, It must have an id field of type int(u64) necessary.
* To make any struct encrypt-able or decrypt-able, It must define sub_key to encrypt and decrypt key individually.
* Before using encrypt-able or decrypt-able, It must have initialize secret_key initially.
* Secret key length depends upon how much good encryption that you want (64 byte recommended).
* It is same as django [encrypted_id](https://pypi.org/project/django-encrypted-id/).
* We can reuse the secret key as we are using in django to en-decrypt the id.

* In version 0.1.4, name changes crate Encrypted to Encrypt, crate Decrypted to Decrypt, 
  function init_encrypt_conf to init_conf, struct function dkey to id. Made it independent crate,
  in earlier version it has a dependency of diesel-mate, In this version removed it. I will make sure
  that in future, I won't change any names, If so I'll deprecate first those.


```rust

#[macro_use]
extern crate encrypted_id_derive;
use encrypted_id::prelude::*;

#[derive(Debug, Default, Encrypt, Decrypt)]
#[encdec_opts(opts(sub_key = "enky_demo_sub_key"))]
pub struct Demo {
    pub id: u64,
    pub name: String,
}

//Deriving Encrypt trait for struct, it will generate a ekey function for this struct.
//Deriving Decrypt trait for struct, it will generate a id function for this struct.

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
```


## Using without struct
```rust

#[test]
fn encrypt_id() {
    init_conf("df(vh!3*8e21@qca#3)w#7ta*z#!bhsde43&#iez3sf5m1#h6l");
    let ekey = encode(5, "sub_key_foo").unwrap();
    let id = decode(&ekey, "sub_key_foo").unwrap();
    assert_eq!("E86VGQhfxb_9rxSfjnBqKg", ekey);
    assert_eq!(5, dkey);
}

``` 

#### *Note Point
We can use this crate with diesel also, just derive Encrypt, Decrypt both trait and define sub_key.
But make sure struct should contain a field `id: u64`. 