#[macro_use]
extern crate lazy_static;

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
