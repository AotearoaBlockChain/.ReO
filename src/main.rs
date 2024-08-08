use ring::digest::{Context, SHA256};
use ring::hmac;
use ring::rand::{SecureRandom, SystemRandom};
use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
use std::error::Error;
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use hex;
use warp::filter
    
mod network;

#[#[tokio::main]
async fn main() {
    // GET /hello => 200 OK with body "Hello, World!"
    let hello = warp::path("hello")
        .map(|| "Hello, World!");

    // GET / => 200 OK with body "Warp server is running!"
    let root = warp::path::end()
        .map(|| "Warp server is running!");

    // Combine the routes
    let routes = hello.or(root);

    // Start the server on port 8080
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}

#[derive(Debug)]
pub enum ReOError {
    IoError(io::Error),
    RingError(ring::error::Unspecified),
    HexError(hex::FromHexError),
}

impl fmt::Display for ReOError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ReOError::IoError(ref err) => write!(f, "IO error: {}", err),
            ReOError::RingError(ref err) => write!(f, "Ring error: {:?}", err),
            ReOError::HexError(ref err) => write!(f, "Hex error: {:?}", err),
        }
    }
}

impl Error for ReOError {}

impl From<io::Error> for ReOError {
    fn from(err: io::Error) -> ReOError {
        ReOError::IoError(err)
    }
}

impl From<ring::error::Unspecified> for ReOError {
    fn from(err: ring::error::Unspecified) -> ReOError {
        ReOError::RingError(err)
    }
}

impl From<hex::FromHexError> for ReOError {
    fn from(err: hex::FromHexError) -> ReOError {
        ReOError::HexError(err)
    }
}

pub fn whakamuka(raraunga: &str) -> Result<String, ReOError> {
    let mut horopaki = Context::new(&SHA256);
    horopaki.update(raraunga.as_bytes());
    let whakamuka = horopaki.finish();
    Ok(hex::encode(whakamuka.as_ref()))
}

pub fn hangaia_hmac(ki: &str, raraunga: &str) -> Result<String, ReOError> {
    let hmac_ki = hmac::Key::new(hmac::HMAC_SHA256, ki.as_bytes());
    let waitohu = hmac::sign(&hmac_ki, raraunga.as_bytes());
    Ok(hex::encode(waitohu.as_ref()))
}

pub fn waihanga_ki() -> Result<String, ReOError> {
    let rng = SystemRandom::new();
    let mut ki = [0u8; 32];
    rng.fill(&mut ki)?;
    Ok(hex::encode(ki))
}

pub fn whakamuna_raraunga_aead(ki_hex: &str, raraunga: &[u8]) -> Result<(Vec<u8>, Vec<u8>), ReOError> {
    let ki = hex::decode(ki_hex)?;
    
    if ki.len() != 32 {
        return Err(ReOError::RingError(ring::error::Unspecified));
    }

    let ki_matapokere = UnboundKey::new(&AES_256_GCM, &ki)?;
    let nonce_purua = {
        let rng = SystemRandom::new();
        let mut nonce = [0u8; 12];
        rng.fill(&mut nonce)?;
        nonce
    };
    let nonce = Nonce::assume_unique_for_key(nonce_purua);
    let mut in_out = raraunga.to_vec();
    let ki_powhiri = LessSafeKey::new(ki_matapokere);
    
    match ki_powhiri.seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out) {
        Ok(()) => Ok((nonce_purua.to_vec(), in_out)),
        Err(e) => Err(ReOError::RingError(e))
    }
}

pub fn wetekina_raraunga_aead(ki_hex: &str, nonce: &[u8], whakamuna: &[u8]) -> Result<Vec<u8>, ReOError> {
    let ki = hex::decode(ki_hex)?;

    if ki.len() != 32 {
        return Err(ReOError::RingError(ring::error::Unspecified));
    }

    let ki_matapokere = UnboundKey::new(&AES_256_GCM, &ki)?;
    let nonce = Nonce::try_assume_unique_for_key(nonce)?;
    let mut in_out = whakamuna.to_vec();
    let ki_powhiri = LessSafeKey::new(ki_matapokere);
    
    match ki_powhiri.open_in_place(nonce, Aad::empty(), &mut in_out) {
        Ok(data) => Ok(data.to_vec()),
        Err(e) => Err(ReOError::RingError(e))
    }
}

pub fn tapirihia_konae(ingoa_konae: &str) -> Result<(), ReOError> {
    let ara = Path::new(ingoa_konae);
    if ara.exists() {
        return Err(ReOError::IoError(io::Error::new(io::ErrorKind::AlreadyExists, "File already exists")));
    }
    File::create(&ara)?;
    Ok(())
}

pub fn mukua_konae(ingoa_konae: &str) -> Result<(), ReOError> {
    let ara = Path::new(ingoa_konae);
    fs::remove_file(&ara)?;
    Ok(())
}

pub fn panuihia_konae(ingoa_konae: &str) -> Result<String, ReOError> {
    let mut ara = File::open(ingoa_konae)?;
    let mut ihirangi = String::new();
    ara.read_to_string(&mut ihirangi)?;
    Ok(ihirangi)
}

pub fn tapirihia_raraunga(ingoa_konae: &str, raraunga: &str) -> Result<(), ReOError> {
    let mut ara = OpenOptions::new().append(true).open(ingoa_konae)?;
    ara.write_all(raraunga.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::fs;
    use std::fs::File;

    #[test]
    fn test_whakamuka() {
        let raraunga = "Hello, world!";
        let result = whakamuka(raraunga);
        assert!(result.is_ok());
        let hash = result.unwrap();
        assert_eq!(hash, "315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3");
    }

    #[test]
    fn test_whakamuka_empty() {
        let raraunga = "";
        let result = whakamuka(raraunga);
        assert!(result.is_ok());
        let hash = result.unwrap();
        assert_eq!(hash, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    }

    #[test]
    fn test_hangaia_hmac() {
        let raraunga = "Hello, world!";
        let ki = "supersecretkey";
        let result = hangaia_hmac(ki, raraunga);
        assert!(result.is_ok());
        let hmac = result.unwrap();
        assert_eq!(hmac, "aa14d38e4aa8e16dc388e4a50e4549779413c834a8076996008e2befe6a873dd");
    }

    #[test]
    fn test_hangaia_hmac_empty_key() {
        let raraunga = "Hello, world!";
        let ki = "";
        let result = hangaia_hmac(ki, raraunga);
        assert!(result.is_ok());
        let hmac = result.unwrap();
        assert_eq!(hmac, "0d192eb5bc5e4407192197cbf9e1658295fa3ff995b3ff914f3cc7c38d83b10f");
    }

    #[test]
    fn test_tapirihia_konae() {
        let ingoa_konae = "testfile.txt";
        if Path::new(ingoa_konae).exists() {
            let _ = fs::remove_file(ingoa_konae);
        }

        let result = tapirihia_konae(ingoa_konae);
        assert!(result.is_ok(), "File creation failed: {:?}", result);
        assert!(Path::new(ingoa_konae).exists(), "File does not exist after creation");
        let _ = fs::remove_file(ingoa_konae);
    }

    #[test]
    fn test_tapirihia_konae_existing_file() {
        let ingoa_konae = "testfile.txt";
        let _ = File::create(ingoa_konae);
        let result = tapirihia_konae(ingoa_konae);
        assert!(result.is_err());
    }

    #[test]
    fn test_mukua_konae() {
        let ingoa_konae = "testfile.txt";
        let _ = File::create(ingoa_konae);
        let result = mukua_konae(ingoa_konae);
        assert!(result.is_ok(), "File deletion failed: {:?}", result);
        assert!(!Path::new(ingoa_konae).exists(), "File still exists after deletion");
    }

    #[test]
    fn test_mukua_konae_nonexistent() {
        let ingoa_konae = "nonexistent.txt";
        let result = mukua_konae(ingoa_konae);
        assert!(result.is_err()); // Expect an error because the file does not exist
    }

    #[test]
    fn test_panuihia_konae() {
        let ingoa_konae = "testfile.txt";
        let content = "This is a test file content.";
        let mut file = File::create(ingoa_konae).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        let result = panuihia_konae(ingoa_konae);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), content);
        let _ = fs::remove_file(ingoa_konae);
    }

    #[test]
    fn test_panuihia_konae_nonexistent() {
        let ingoa_konae = "nonexistent.txt";
        let result = panuihia_konae(ingoa_konae);
        assert!(result.is_err()); // Expect an error because the file does not exist
    }

    #[test]
    fn test_tapirihia_raraunga() {
        let ingoa_konae = "testfile.txt";
        let _ = File::create(ingoa_konae);
        let raraunga = "This is some test data.";
        let result = tapirihia_raraunga(ingoa_konae, raraunga);
        assert!(result.is_ok(), "Appending data to file failed: {:?}", result);
        let mut file_content = String::new();
        let mut file = File::open(ingoa_konae).unwrap();
        file.read_to_string(&mut file_content).unwrap();
        assert!(file_content.contains(raraunga), "File content does not match expected data");
        let _ = fs::remove_file(ingoa_konae);
    }

    #[test]
    fn test_waihanga_ki() {
        let result = waihanga_ki();
        assert!(result.is_ok());
        let key = result.unwrap();
        assert_eq!(key.len(), 64); // 32 bytes = 64 hex characters
    }

    #[test]
    fn test_whakamuna_raraunga_aead() {
        let key = waihanga_ki().unwrap();
        let raraunga = "Sensitive data";
        let result = whakamuna_raraunga_aead(&key, raraunga.as_bytes());
        assert!(result.is_ok(), "Data encryption failed: {:?}", result);
    }

    #[test]
    fn test_wetekina_raraunga_aead() {
        let key = waihanga_ki().unwrap();
        let raraunga = "Sensitive data";
        let (nonce, encrypted_data) = whakamuna_raraunga_aead(&key, raraunga.as_bytes()).unwrap();
        let result = wetekina_raraunga_aead(&key, &nonce, &encrypted_data);
        assert!(result.is_ok(), "Data decryption failed: {:?}", result);
        assert_eq!(result.unwrap(), raraunga.as_bytes());
    }
        }
