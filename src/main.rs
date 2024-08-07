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

mod network;

#[tokio::main]
async fn main() {
    // Call the function to run the HTTP server
    network::run_server().await;
}

// Custom error type
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

// Hash data
pub fn whakamuka(raraunga: &str) -> Result<String, ReOError> {
    let mut horopaki = Context::new(&SHA256);
    horopaki.update(raraunga.as_bytes());
    let whakamuka = horopaki.finish();
    Ok(hex::encode(whakamuka.as_ref()))
}

// Create HMAC
pub fn hangaia_hmac(ki: &str, raraunga: &str) -> Result<String, ReOError> {
    let hmac_ki = hmac::Key::new(hmac::HMAC_SHA256, ki.as_bytes());
    let waitohu = hmac::sign(&hmac_ki, raraunga.as_bytes());
    Ok(hex::encode(waitohu.as_ref()))
}

// Generate a random key
pub fn waihanga_ki() -> Result<String, ReOError> {
    let rng = SystemRandom::new();
    let mut ki = [0u8; 32];
    rng.fill(&mut ki)?;
    Ok(hex::encode(ki))
}

// Encrypt data
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

// Decrypt data
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

// Add a file
pub fn tapirihia_konae(ingoa: &str) -> Result<(), ReOError> {
    let ara = Path::new(ingoa);
    if ara.exists() {
        return Err(ReOError::IoError(io::Error::new(io::ErrorKind::AlreadyExists, "Konae already exists")));
    }
    File::create(&ara)?;
    Ok(())
}

// Delete a file
pub fn mukua_konae(ingoa: &str) -> Result<(), ReOError> {
    let ara = Path::new(ingoa);
    fs::remove_file(&ara)?;
    Ok(())
}

// Read from a file
pub fn panuihia_konae(ingoa: &str) -> Result<String, ReOError> {
    let mut ara = File::open(ingoa)?;
    let mut ihirangi = String::new();
    ara.read_to_string(&mut ihirangi)?;
    Ok(ihirangi)
}

// Append data to a file
pub fn tapirihia_raraunga(ingoa: &str, raraunga: &str) -> Result<(), ReOError> {
    let mut ara = OpenOptions::new().append(true).open(ingoa)?;
    ara.write_all(raraunga.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    use uuid::Uuid;

    #[test]
    fn test_whakamuka() {
        let raraunga = "Hello, world!";
        let result = whakamuka(raraunga);
        assert!(result.is_ok());
        let hash = result.unwrap();
        assert_eq!(hash, "315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3"); // Expected hash for "Hello, world!"
    }

    #[test]
    fn test_whakamuka_empty() {
        let raraunga = "";
        let result = whakamuka(raraunga);
        assert!(result.is_ok());
        let hash = result.unwrap();
        assert_eq!(hash, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"); // Expected hash for empty string
    }

    #[test]
    fn test_hangaia_hmac() {
        let raraunga = "Hello, world!";
        let ki = "supersecretkey";
        let result = hangaia_hmac(ki, raraunga);
        assert!(result.is_ok());
        let hmac = result.unwrap();
        assert_eq!(hmac, "aa14d38e4aa8e16dc388e4a50e4549779413c834a8076996008e2befe6a873dd"); // Expected HMAC for "Hello, world!" with key "supersecretkey"
    }

    #[test]
    fn test_hangaia_hmac_empty_key() {
        let raraunga = "Hello, world!";
        let ki = "";
        let result = hangaia_hmac(ki, raraunga);
        assert!(result.is_ok());
        let hmac = result.unwrap();
        assert_eq!(hmac, "0d192eb5bc5e4407192197cbf9e1658295fa3ff995b3ff914f3cc7c38d83b10f"); // Expected HMAC for "Hello, world!" with empty key
    }

    #[test]
    fn test_tapirihia_konae() {
        let ingoa_konae = "testfile.txt";
        let result = tapirihia_konae(ingoa_konae);
        assert!(result.is_ok());
        assert!(Path::new(ingoa_konae).exists());
        let _ = fs::remove_file(ingoa_konae); // Clean up
    }

    #[test]
    fn test_tapirihia_konae_existing_file() {
        let ingoa_konae = "testfile.txt";
        let _ = File::create(ingoa_konae);
        let result = tapirihia_konae(ingoa_konae);
        assert!(result.is_err()); // Expect an error because the file already exists
        let _ = fs::remove_file(ingoa_konae); // Clean up
    }

    #[test]
    fn test_mukua_konae() {
        let ingoa_konae = "testfile.txt";
        let _ = File::create(ingoa_konae); // Ensure the file exists
        let result = mukua_konae(ingoa_konae);
        assert!(result.is_ok());
        assert!(!Path::new(ingoa_konae).exists());
    }

    #[test]
    fn test_mukua_konae_nonexistent() {
        let ingoa_konae = "nonexistentfile.txt";
        let result = mukua_konae(ingoa_konae);
        assert!(result.is_err()); // Expect an error since the file doesn't exist
    }

    #[test]
    fn test_panuihia_konae() {
        let unique_file_name = format!("testfile_{}.txt", Uuid::new_v4());
        let ingoa_konae = unique_file_name.as_str();
        let ihirangi = "This is a test content.";

        // Ensure previous file (if any) is removed
        if Path::new(ingoa_konae).exists() {
            let _ = fs::remove_file(ingoa_konae);
        }

        // Write content to the file
        let write_result = fs::write(ingoa_konae, ihirangi);
        assert!(write_result.is_ok(), "Failed to write to test file: {:?}", write_result);

        // Wait a bit to ensure file system operations complete
        thread::sleep(Duration::from_millis(10));

        // Ensure file write has been successful before reading it
        let result = panuihia_konae(ingoa_konae);
        assert!(result.is_ok(), "Failed to read the file: {:?}", result);
        let ihirangi_konae = result.unwrap();
        assert_eq!(ihirangi_konae, ihirangi, "File content does not match expected content");

        // Clean up
        let cleanup_result = fs::remove_file(ingoa_konae);
        assert!(cleanup_result.is_ok(), "Failed to clean up test file: {:?}", cleanup_result);
    }

    #[test]
    fn test_panuihia_konae_nonexistent() {
        let ingoa_konae = "nonexistentfile.txt";
        let result = panuihia_konae(ingoa_konae);
        assert!(result.is_err()); // Expect an error since the file doesn't exist
    }

    #[test]
    fn test_tapirihia_raraunga() {
        let unique_file_name = format!("testfile_{}.txt", Uuid::new_v4());
        let ingoa_konae = unique_file_name.as_str();
        let ihirangi_tuakiri = "Initial content.";
        let ihirangi_tapiri = " Appended content.";

        // Write initial content to the file
        let _ = fs::write(ingoa_konae, ihirangi_tuakiri);

        // Append content to the file
        let result = tapirihia_raraunga(ingoa_konae, ihirangi_tapiri);
        assert!(result.is_ok(), "Failed to append data to file: {:?}", result);

        // Read the file to check content
        let ihirangi_konae = fs::read_to_string(ingoa_konae).unwrap();
        assert_eq!(ihirangi_konae, format!("{}{}", ihirangi_tuakiri, ihirangi_tapiri), "File content does not match expected appended content");

        // Clean up
        let _ = fs::remove_file(ingoa_konae);
    }

    #[test]
    fn test_waihanga_ki() {
        let result = waihanga_ki();
        assert!(result.is_ok());
        let ki = result.unwrap();
        assert_eq!(ki.len(), 64); // Expected length for a 256-bit key in hex
    }

    #[test]
    fn test_whakamuna_raraunga_aead() {
        let ki = waihanga_ki().unwrap();
        let raraunga = "Sensitive data.";
        let (nonce, whakamuna) = whakamuna_raraunga_aead(&ki, raraunga.as_bytes()).unwrap();
        assert!(whakamuna.len() > 0); // Ensure encryption was successful
    }

    #[test]
    fn test_wetekina_raraunga_aead() {
        let ki = waihanga_ki().unwrap();
        let raraunga = "Sensitive data.";
        let (nonce, whakamuna) = whakamuna_raraunga_aead(&ki, raraunga.as_bytes()).unwrap();
        let wetekina = wetekina_raraunga_aead(&ki, &nonce, &whakamuna);
        assert!(wetekina.is_ok(), "Decryption failed: {:?}", wetekina);
        let raraunga_wetekina = String::from_utf8(wetekina.unwrap());
        assert!(raraunga_wetekina.is_ok(), "Failed to convert decrypted data to string");
        let raraunga_wetekina = raraunga_wetekina.unwrap();
        assert_eq!(raraunga_wetekina, raraunga, "Decrypted data does not match original data");
    }
            }
