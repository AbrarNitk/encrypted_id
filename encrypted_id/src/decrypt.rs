use aes::Aes256;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use byteorder::ReadBytesExt;
use cbc::Decryptor;
use cipher::{BlockDecryptMut, KeyIvInit};
use sha2::{Digest, Sha256};

type Aes256CbcDec = Decryptor<Aes256>;

pub trait Decrypt {
    fn id(&self, ekey: &str) -> crate::EResult<u64>;
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
    let emsg = match URL_SAFE.decode(&ekey) {
        Ok(m) => m,
        Err(_) => return Err(crate::EError::InvalidInput.into()),
    };

    // let mut sha = crypto::sha2::Sha256::new();
    // sha.input_str(&format!("{}{}", secret_key, sub_key));
    // let mut iv: Vec<u8> = vec![0; 32];
    // sha.result(&mut iv);
    // let iv = &iv[..16];

    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}", secret_key, sub_key));
    let hash = hasher.finalize();
    let iv = &hash[..16];

    /*
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
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true);
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
    */

    let mut decryptor = Aes256CbcDec::new_from_slices(&secret_key_bytes[..32], iv)
        .map_err(|e| crate::EError::Decrypt(e.to_string()))?;

    if emsg.len() != 16 {
        return Err(crate::EError::InvalidInput.into());
    }

    let mut block = cipher::generic_array::GenericArray::clone_from_slice(&emsg);
    decryptor.decrypt_block_mut(&mut block);
    let final_result = block;

    let mut rdr = std::io::Cursor::new(final_result);
    let crc = rdr.read_u32::<byteorder::LittleEndian>()?;
    let id = rdr.read_u64::<byteorder::LittleEndian>()?;
    let version = rdr.read_u32::<byteorder::LittleEndian>()?;

    let crc_calc = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
    let expected_crc: u32 = if version == 0 {
        crc_calc.checksum(&vec![0; id as usize])
    } else {
        let id: String = id.to_string();
        let id_bytes = id.as_bytes();
        crc_calc.checksum(id_bytes)
    };

    if crc != expected_crc {
        return Err(crate::EError::CRCMismatch.into());
    }
    Ok(id)
}

#[deprecated(since = "0.1.5", note = "Please use .decrypt() instead")]
pub fn decode(ekey: &str, sub_key: &str) -> crate::EResult<u64> {
    decrypt(ekey, sub_key)
}

pub fn decrypt(ekey: &str, sub_key: &str) -> crate::EResult<u64> {
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

#[deprecated(since = "0.1.5", note = "Please use .decrypt_with_secret() instead")]
pub fn decode_with_secret(ekey: &str, sub_key: &str, secret: &str) -> crate::EResult<u64> {
    decrypt_with_secret(ekey, sub_key, secret)
}

pub fn decrypt_with_secret(ekey: &str, sub_key: &str, secret: &str) -> crate::EResult<u64> {
    decode_util(ekey, sub_key, secret, secret.as_bytes().as_ref())
}
