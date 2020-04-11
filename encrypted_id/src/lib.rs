#[macro_use]
extern crate lazy_static;
use failure::Fail;

pub type EResult<T> = std::result::Result<T, failure::Error>;

mod encrypt;
mod decrypt;

#[derive(Default)]
pub struct Config {
    secret_key: Option<String>,
    secret_key_bytes: Vec<u8>,
}

lazy_static! {
    pub(crate) static ref CONFIG: std::sync::RwLock<Config> = std::sync::RwLock::new(Config::default());
}

pub fn init_conf(secret_key: &str) {
    let mut conf = CONFIG.write().unwrap();
    conf.secret_key = Some(secret_key.to_string());
    conf.secret_key_bytes = secret_key.as_bytes().to_owned();
}

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Encryption error: {:?}", _0)]
    Encrypt(crypto::symmetriccipher::SymmetricCipherError),

    #[fail(display = "Decryption error: {:?}", _0)]
    Decrypt(crypto::symmetriccipher::SymmetricCipherError),

    #[fail(display = "Invalid input")]
    InvalidInput,

    #[fail(display = "CRC mismatch")]
    CRCMismatch,

    #[fail(display = "SecretKey is none in encrypt config, initialize config first")]
    SecretKeyNotFound,
}
