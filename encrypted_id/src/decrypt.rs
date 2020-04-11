pub trait Decrypted {
    fn id(&self, ekey: &str) -> crate::Result<u64>;
}