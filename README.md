# Encrypted ID

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/encrypted_id)](https://crates.io/crates/encrypted_id)
[![Build Status](https://travis-ci.org/AbrarNitk/encrypted_id.svg?branch=master)](https://travis-ci.org/AbrarNitk/encrypted_id)

Read more about this library on: [fifthtry.com/abrar/encrypted_id/](https://www.fifthtry.com/abrar/encrypted_id/).

#### Usage

```toml
[dependencies]
encrypted_id = "0.1.5"
```

```rust
fn main() {
    encrypted_id::init("df(vh!3*8e21@qca#3)w#7ta*z#!bhsde43&#iez3sf5m1#h6l");
    let ekey = encrypted_id::encrypt(5, "sub_key_foo").unwrap();
    let id = encrypted_id::decrypt(&ekey, "sub_key_foo").unwrap();
    assert_eq!("E86VGQhfxb_9rxSfjnBqKg", ekey);
    assert_eq!(5, dkey);
}
```
