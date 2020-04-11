pub trait Decrypted {
    fn id(
        &self,
        ekey: &str,
    ) -> crate::EResult<u64>;
}
