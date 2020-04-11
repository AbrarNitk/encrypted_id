pub trait Encrypted {
    fn ekey(&self) -> crate::EResult<String>;
}