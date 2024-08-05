use ring::digest::{Context, SHA256};
use ring::hmac;
use ring::rand::SystemRandom;
use ring::signature::{self, EcdsaKeyPair, KeyPair, ECDSA_P256_SHA256_FIXED_SIGNING};
use std::error::Error;
use std::fs::{self, File};
use std::path::Path;
use hex;

// Whakamuna raraunga (Hash data)
pub fn whakamuna_raraunga(raraunga: &str) -> Result<String, Box<dyn Error>> {
    let mut horopaki = Context::new(&SHA256);
    horopaki.update(raraunga.as_bytes());
    let whakamuna = horopaki.finish();
    Ok(hex::encode(whakamuna.as_ref()))
}

// Waihangahia te HMAC (Create HMAC)
pub fn hangaia_hmac(ki: &str, raraunga: &str) -> Result<String, Box<dyn Error>> {
    let hmac_ki = hmac::Key::new(hmac::HMAC_SHA256, ki.as_bytes());
    let waitohu = hmac::sign(&hmac_ki, raraunga.as_bytes());
    Ok(hex::encode(waitohu.as_ref()))
}

// Tapirihia he konae (Add a file)
pub fn tapirihia_konae(ingoa: &str) -> Result<(), Box<dyn Error>> {
    let ara = Path::new(ingoa);
    File::create(&ara)?;
    println!("Konae '{}' kua tapirihia", ingoa);
    Ok(())
}

// Mukua he konae (Delete a file)
pub fn mukua_konae(ingoa: &str) -> Result<(), Box<dyn Error>> {
    let ara = Path::new(ingoa);
    fs::remove_file(&ara)?;
    println!("Konae '{}' kua mukua", ingoa);
    Ok(())
}

// Main function
fn main() {
    // Tauira whakamahi (Example usage of the functions)
    let raraunga = "Hello, world!";
    match whakamuna_raraunga(raraunga) {
        Ok(hash) => println!("Whakamuna: {}", hash),
        Err(e) => eprintln!("Hapa whakamuna raraunga: {}", e),
    }

    let ki = "supersecretkey";
    match hangaia_hmac(ki, raraunga) {
        Ok(hmac) => println!("HMAC: {}", hmac),
        Err(e) => eprintln!("Hapa hanga HMAC: {}", e),
    }

    let ingoa_konae = "tauira.txt";
    match tapirihia_konae(ingoa_konae) {
        Ok(()) => println!("Konae kua tapirihia: '{}'", ingoa_konae),
        Err(e) => eprintln!("Hapa tapiri konae: {}", e),
    }

    match mukua_konae(ingoa_konae) {
        Ok(()) => println!("Konae kua mukua: '{}'", ingoa_konae),
        Err(e) => eprintln!("Hapa muku konae: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whakamuna_raraunga() {
        let data = "Hello, world!";
        let result = whakamuna_raraunga(data);
        assert!(result.is_ok());
        let hash = result.unwrap();
        assert_eq!(hash, "315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3"); // Expected hash for "Hello, world!"
    }

    #[test]
    fn test_whakamuna_raraunga_empty() {
        let data = "";
        let result = whakamuna_raraunga(data);
        assert!(result.is_ok());
        let hash = result.unwrap();
        assert_eq!(hash, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"); // Expected hash for empty string
    }

    #[test]
    fn test_hangaia_hmac() {
        let data = "Hello, world!";
        let key = "supersecretkey";
        let result = hangaia_hmac(key, data);
        assert!(result.is_ok());
        let hmac = result.unwrap();
        assert_eq!(hmac, "aa14d38e4aa8e16dc388e4a50e4549779413c834a8076996008e2befe6a873dd"); // Expected HMAC for "Hello, world!" with key "supersecretkey"
    }

    #[test]
    fn test_hangaia_hmac_empty_key() {
        let data = "Hello, world!";
        let key = "";
        let result = hangaia_hmac(key, data);
        assert!(result.is_ok());
        let hmac = result.unwrap();
        assert_eq!(hmac, "63c8f2b432c649da5b788e660e8a98f6497d89c85f8795fbc4e57911d123cdf8"); // Expected HMAC for "Hello, world!" with empty key
    }

    #[test]
    fn test_tapirihia_konae() {
        let filename = "testfile.txt";
        let result = tapirihia_konae(filename);
        assert!(result.is_ok());
        assert!(Path::new(filename).exists());
        let _ = fs::remove_file(filename); // Clean up
    }

    #[test]
    fn test_tapirihia_konae_existing_file() {
        let filename = "testfile.txt";
        let _ = File::create(filename);
        let result = tapirihia_konae(filename);
        assert!(result.is_err()); // Expect an error because the file already exists
        let _ = fs::remove_file(filename); // Clean up
    }

    #[test]
    fn test_mukua_konae() {
        let filename = "testfile.txt";
        let _ = File::create(filename); // Ensure the file exists
        let result = mukua_konae(filename);
        assert!(result.is_ok());
        assert!(!Path::new(filename).exists());
    }

    #[test]
    fn test_mukua_konae_nonexistent() {
        let filename = "nonexistentfile.txt";
        let result = mukua_konae(filename);
        assert!(result.is_err()); // Expect an error since the file doesn't exist
    }

    #[test]
    fn test_mukua_konae_permission_denied() {
        let filename = "/root/testfile.txt";
        let result = mukua_konae(filename);
        assert!(result.is_err()); // Expect an error due to permission denied
    }

    #[test]
    fn test_large_input_whakamuna() {
        let data = "a".repeat(10_000);
        let result = whakamuna_raraunga(&data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_large_input_hangaia_hmac() {
        let data = "a".repeat(10_000);
        let key = "supersecretkey";
        let result = hangaia_hmac(key, &data);
        assert!(result.is_ok());
    }
                }
