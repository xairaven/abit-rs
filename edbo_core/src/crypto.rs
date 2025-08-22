use aes::Aes256;
use base64::Engine;
use base64::engine::general_purpose;
use block_padding::Pkcs7;
use cbc::cipher::{BlockDecryptMut, KeyIvInit};
use sha2::{Digest, Sha256};
use thiserror::Error;

type Aes256CbcDec = cbc::Decryptor<Aes256>;

fn decrypt(fio: String, number: i32, prsid: i32) -> Result<String, CryptoError> {
    const CUSTOM_KEY: &str = "2025";
    let multiply_key = multiply(number, prsid);

    let k = format!("{:x}", Sha256::digest(multiply_key));
    let k = &k[..k.len().min(32)];

    let i = format!("{:x}", Sha256::digest(CUSTOM_KEY));

    let e = general_purpose::STANDARD
        .decode(fio)
        .map_err(CryptoError::FailedBase64Decode)?;
    let e = String::from_utf8(e).map_err(CryptoError::InvalidUtf8)?;

    let encrypted_data = general_purpose::STANDARD
        .decode(e)
        .map_err(CryptoError::FailedBase64Decode)?;

    let key = k.as_bytes();
    let iv = i.as_bytes();

    let mut key_32 = [0u8; 32];
    let key_len = std::cmp::min(key.len(), 32);
    key_32
        .get_mut(..key_len)
        .ok_or(CryptoError::Slicing)?
        .copy_from_slice(key.get(..key_len).ok_or(CryptoError::Slicing)?);

    let mut iv_16 = [0u8; 16];
    let iv_len = std::cmp::min(iv.len(), 16);
    iv_16
        .get_mut(..iv_len)
        .ok_or(CryptoError::Slicing)?
        .copy_from_slice(iv.get(..iv_len).ok_or(CryptoError::Slicing)?);

    let cipher = Aes256CbcDec::new(&key_32.into(), &iv_16.into());

    let mut buffer = encrypted_data;
    let decrypted_data = cipher
        .decrypt_padded_mut::<Pkcs7>(&mut buffer)
        .map_err(CryptoError::WrongPad)?;

    let d =
        String::from_utf8(decrypted_data.to_vec()).map_err(CryptoError::InvalidUtf8)?;

    Ok(d)
}

fn multiply(number: i32, prsid: i32) -> String {
    let subtract_value = subtract(prsid);
    format!("v{}", number * subtract_value)
}

fn subtract(prsid: i32) -> i32 {
    const KEY: i32 = 7500;
    KEY - prsid
}

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("Failed to decode Base64. {0}")]
    FailedBase64Decode(base64::DecodeError),

    #[error("Invalid UTF8. {0}")]
    InvalidUtf8(std::string::FromUtf8Error),

    #[error("Wrong IV length.")]
    WrongIV,

    #[error("Wrong key length, must be 16 or 32 bytes.")]
    UnsupportedKeyLength,

    #[error("Wrong padding.")]
    WrongPad(block_padding::UnpadError),

    #[error("Wrong indexing.")]
    Slicing,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let fio = "MGRoaTJ5eFE3d05GWU0vNjVDcmlJNUNkb3FRZk5nQnhmUTF5ZVh1RDNDaz0=";
        let number = 1;
        let prsid = 6;

        let dec = decrypt(fio.to_string(), number, prsid).unwrap();
        assert_eq!(dec, "Ковальов О. О.");
    }

    #[test]
    fn test2() {
        let fio = "UjFiVWJrMUlGOTkwbXlqSVNkZ21nOURXdDBRNFFMODNwaS84MlM3eG5Kaz0=";
        let number = 4;
        let prsid = 5;

        let dec = decrypt(fio.to_string(), number, prsid).unwrap();
        assert_eq!(dec, "Карбан К. А.");
    }

    #[test]
    fn test3() {
        let fio = "TTdMQmt4ZkFlN2JqZnA1L1ZZMkhUSmsyL3FrSU53UHRJdGcvMnFnaUV6bz0=";
        let p = "N1dtV2NNSmkrRjlSWnV5cmJkSWd3UT09";
        let number = 7;
        let prsid = 6;

        let fio = decrypt(fio.to_string(), number, prsid).unwrap();
        assert_eq!(fio, "Дем`янчук О. П.");
        let p = decrypt(p.to_string(), number, prsid).unwrap();
        assert_eq!(p, "4 (Б)");
    }
}
