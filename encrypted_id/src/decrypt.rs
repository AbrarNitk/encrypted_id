use byteorder::ReadBytesExt;
use crypto::{
    buffer::{ReadBuffer, WriteBuffer},
    digest::Digest,
};

pub trait Decrypt {
    fn id(
        &self,
        ekey: &str,
    ) -> crate::EResult<u64>;
}

fn decode_util(
    ekey: &str,
    sub_key: &str,
    secret_key: &str,
    secret_key_bytes: &[u8],
) -> crate::EResult<u64> {
    if ekey.is_empty() {
        return Err(crate::EError::InvalidInput.into());
    }

    let ekey = ekey.to_string();
    let padding: String = vec!['='; 3 - ekey.len() % 3].into_iter().collect();
    let ekey = ekey + &padding;
    let emsg = match base64::decode_config(&ekey, base64::URL_SAFE) {
        Ok(m) => m,
        Err(_) => return Err(crate::EError::InvalidInput.into()),
    };

    let mut sha = crypto::sha2::Sha256::new();
    sha.input_str(&format!("{}{}", secret_key, sub_key));
    let mut iv: Vec<u8> = vec![0; 32];
    sha.result(&mut iv);
    let iv = &iv[..16];

    let mut decryptor = crypto::aes::cbc_decryptor(
        crypto::aes::KeySize::KeySize256,
        &secret_key_bytes[..32],
        iv,
        crypto::blockmodes::NoPadding,
    );

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(&emsg);
    let mut buffer = [0; 16];
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result =
            decryptor.decrypt(&mut read_buffer, &mut write_buffer, true);
        let result = match result {
            Ok(v) => v,
            Err(e) => return Err(crate::EError::Decrypt(e).into()),
        };
        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .copied(),
        );

        match result {
            crypto::buffer::BufferResult::BufferUnderflow => break,
            crypto::buffer::BufferResult::BufferOverflow => {}
        }
    }

    let mut rdr = std::io::Cursor::new(final_result);
    let crc = rdr.read_u32::<byteorder::LittleEndian>()? & 0xffff_ffff;
    let id = rdr.read_u64::<byteorder::LittleEndian>()?;
    let version = rdr.read_u32::<byteorder::LittleEndian>()?;

    let expected_crc: u32 = if version == 0 {
        crc::crc32::checksum_ieee(&vec![0; id as usize]) & 0xffff_ffff
    } else {
        let id: String = id.to_string();
        let id_bytes = id.as_bytes();
        crc::crc32::checksum_ieee(id_bytes) & 0xffff_ffff
    };

    if crc != expected_crc {
        return Err(crate::EError::CRCMismatch.into());
    }
    Ok(id)
}

pub fn decode(
    ekey: &str,
    sub_key: &str,
) -> crate::EResult<u64> {
    let config = crate::CONFIG.read().unwrap();
    if config.secret_key.is_none() {
        return Err(crate::EError::SecretKeyNotFound.into());
    }
    decode_util(
        ekey,
        sub_key,
        config.secret_key.as_ref().unwrap(),
        config.secret_key_bytes.as_ref(),
    )
}

pub fn decode_with_secret(
    ekey: &str,
    sub_key: &str,
    secret_key: &str,
) -> crate::EResult<u64> {
    decode_util(ekey, sub_key, secret_key, secret_key.as_bytes().as_ref())
}
