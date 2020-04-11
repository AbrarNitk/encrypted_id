use base64;
use byteorder::WriteBytesExt;
use crypto::{
    buffer::{ReadBuffer, WriteBuffer},
    digest::Digest,
};

pub trait Encrypted {
    fn ekey(&self) -> crate::EResult<String>;
}

fn encode_id_util(
    id: u64,
    sub_key: &str,
    secret_key: &str,
    secret_key_bytes: &[u8],
) -> crate::EResult<String> {
    let version: u32 = 1;
    let crc: u32 =
        crc::crc32::checksum_ieee(id.to_string().as_bytes()) & 0xffffffff;

    let mut msg: Vec<u8> = vec![];
    msg.write_u32::<byteorder::LittleEndian>(crc)?;
    msg.write_u64::<byteorder::LittleEndian>(id)?;
    msg.write_u32::<byteorder::LittleEndian>(version)?;

    let mut sha_value = crypto::sha2::Sha256::new();
    sha_value.input_str(&format!("{}{}", secret_key, sub_key));
    let mut iv: Vec<u8> = vec![0; 32];
    sha_value.result(&mut iv);
    let iv = &iv[..16];
    let mut encryptor = crypto::aes::cbc_encryptor(
        crypto::aes::KeySize::KeySize256,
        &secret_key_bytes[..32],
        iv,
        crypto::blockmodes::NoPadding,
    );

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(msg.as_ref());
    let mut buffer = [0; 16];
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result =
            encryptor.encrypt(&mut read_buffer, &mut write_buffer, true);

        let result = match result {
            Ok(v) => v,
            Err(e) => return Err(crate::EError::Encrypt(e).into()),
        };

        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );

        match result {
            crypto::buffer::BufferResult::BufferUnderflow => break,
            crypto::buffer::BufferResult::BufferOverflow => {}
        }
    }
    Ok(base64::encode_config(&final_result, base64::URL_SAFE).replace("=", ""))
}

pub fn encode(
    id: u64,
    sub_key: &str,
) -> crate::EResult<String> {
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

pub fn encode_with_secret(
    id: u64,
    sub_key: &str,
    secret_key: &str,
) -> crate::EResult<String> {
    encode_id_util(id, sub_key, secret_key, secret_key.as_bytes().as_ref())
}
