#[macro_use]
extern crate lazy_static;

use failure::Fail;

pub type EResult<T> = std::result::Result<T, failure::Error>;

pub mod decrypt;
pub mod encrypt;
pub mod prelude;

pub use crate::decrypt::decrypt;
pub use crate::encrypt::encrypt;

#[derive(Default)]
struct Config {
    secret_key: Option<String>,
    secret_key_bytes: Vec<u8>,
}

lazy_static! {
    pub(crate) static ref CONFIG: std::sync::RwLock<Config> =
        std::sync::RwLock::new(Config::default());
}

#[deprecated(since = "0.1.5", note = "Please use .init() instead")]
pub fn init_conf(secret_key: &str) {
    init(secret_key)
}

pub fn init(secret_key: &str) {
    let mut conf = CONFIG.write().unwrap();
    conf.secret_key = Some(secret_key.to_string());
    conf.secret_key_bytes = secret_key.as_bytes().to_owned();
}

#[derive(Fail, Debug)]
pub enum EError {
    #[fail(display = "Encryption error: {:?}", _0)]
    Encrypt(crypto::symmetriccipher::SymmetricCipherError),

    #[fail(display = "Decryption error: {:?}", _0)]
    Decrypt(crypto::symmetriccipher::SymmetricCipherError),

    #[fail(display = "Invalid input")]
    InvalidInput,

    #[fail(display = "CRC mismatch")]
    CRCMismatch,

    #[fail(
        display = "SecretKey is none in encrypt config, initialize config first"
    )]
    SecretKeyNotFound,
}
