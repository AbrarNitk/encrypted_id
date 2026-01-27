use aes::Aes256;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use byteorder::WriteBytesExt;
use cbc::Encryptor;
use cipher::{BlockEncryptMut, KeyIvInit};
use sha2::{Digest, Sha256};

type Aes256CbcEnc = Encryptor<Aes256>;

pub trait Encrypt {
    fn ekey(&self) -> crate::EResult<String>;
}

fn encode_id_util(
    id: u64,
    sub_key: &str,
    secret_key: &str,
    secret_key_bytes: &[u8],
) -> crate::EResult<String> {
    let version: u32 = 1;
    let crc = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
    let checksum: u32 = crc.checksum(id.to_string().as_bytes());

    let mut msg: Vec<u8> = vec![];
    msg.write_u32::<byteorder::LittleEndian>(checksum)?;
    msg.write_u64::<byteorder::LittleEndian>(id)?;
    msg.write_u32::<byteorder::LittleEndian>(version)?;

    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}", secret_key, sub_key));
    let hash = hasher.finalize();
    let iv = &hash[..16];

    let mut encryptor = Aes256CbcEnc::new_from_slices(&secret_key_bytes[..32], iv)
        .map_err(|e| crate::EError::Encrypt(e.to_string()))?;

    // msg is exactly 16 bytes (4+8+4), so one block.
    // We can cast the slice to a GenericArray for the block.
    let mut block = cipher::generic_array::GenericArray::clone_from_slice(&msg);
    encryptor.encrypt_block_mut(&mut block);

    Ok(URL_SAFE.encode(&block).replace("=", ""))
}

#[deprecated(since = "0.1.5", note = "Please use .encrypt() instead")]
pub fn encode(id: u64, sub_key: &str) -> crate::EResult<String> {
    encrypt(id, sub_key)
}

pub fn encrypt(id: u64, sub_key: &str) -> crate::EResult<String> {
    let config = crate::CONFIG.read().unwrap();
    if config.secret_key.is_none() {
        return Err(crate::EError::SecretKeyNotFound.into());
    }
    encode_id_util(
        id,
        sub_key,
        config.secret_key.as_ref().unwrap(),
        config.secret_key_bytes.as_ref(),
    )
}

#[deprecated(since = "0.1.5", note = "Please use .encrypt_with_secret() instead")]
pub fn encode_with_secret(id: u64, sub_key: &str, secret: &str) -> crate::EResult<String> {
    encrypt_with_secret(id, sub_key, secret)
}

pub fn encrypt_with_secret(id: u64, sub_key: &str, secret: &str) -> crate::EResult<String> {
    encode_id_util(id, sub_key, secret, secret.as_bytes().as_ref())
}
