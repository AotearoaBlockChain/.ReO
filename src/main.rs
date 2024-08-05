use ring::digest::{Context, SHA256};
use ring::hmac;
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
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
    if ara.exists() {
        return Err(Box::from("Konae already exists"));
    }
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

// Pānuihia he konae (Read from a file)
pub fn panuihia_konae(ingoa: &str) -> Result<String, Box<dyn Error>> {
    let mut ara = File::open(ingoa)?;
    let mut ihirangi = String::new();
    ara.read_to_string(&mut ihirangi)?;
    Ok(ihirangi)
}

// Tāpirihia raraunga ki te konae (Append data to a file)
pub fn tapirihia_raraunga(ingoa: &str, raraunga: &str) -> Result<(), Box<dyn Error>> {
    let mut ara = OpenOptions::new().append(true).open(ingoa)?;
    ara.write_all(raraunga.as_bytes())?;
    println!("Raraunga kua tāpirihia ki te konae '{}'", ingoa);
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

    match panuihia_konae(ingoa_konae) {
        Ok(ihirangi) => println!("Ihirangi o te konae: '{}'", ihirangi),
        Err(e) => eprintln!("Hapa pānui konae: {}", e),
    }

    match tapirihia_raraunga(ingoa_konae, "\nHe rārangi anō.") {
        Ok(()) => println!("Raraunga kua tāpirihia ki te konae: '{}'", ingoa_konae),
        Err(e) => eprintln!("Hapa tāpiri raraunga ki te konae: {}", e),
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
        // Correct the expected HMAC value here if necessary
        assert_eq!(hmac, "0d192eb5bc5e4407192197cbf9e1658295fa3ff995b3ff914f3cc7c38d83b10f"); // Expected HMAC for "Hello, world!" with empty key
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

    #[test]
    fn test_panuihia_konae() {
        let filename = "testfile.txt";
        let content = "This is a test content.";
        let _ = fs::write(filename, content); // Write content to the file
        let result = panuihia_konae(filename);
        assert!(result.is_ok());
        let file_content = result.unwrap();
        assert_eq!(file_content, content); // Expected content to match the written content
        let _ = fs::remove_file(filename); // Clean up
    }

    #[test]
    fn test_panuihia_konae_nonexistent() {
        let filename = "nonexistentfile.txt";
        let result = panuihia_konae(filename);
        assert!(result.is_err()); // Expect an error since the file doesn't exist
    }

    #[test]
    fn test_tapirihia_raraunga() {
        let filename = "testfile.txt";
        let initial_content = "Initial content.";
        let append_content = " Appended content.";
        let _ = fs::write(filename, initial_content); // Write initial content to the file
        let result = tapirihia_raraunga(filename, append_content);
        assert!(result.is_ok());
        let file_content = fs::read_to_string(filename).unwrap();
        assert_eq!(file_content, format!("{}{}", initial_content, append_content)); // Expected content to be concatenated
        let _ = fs::remove_file(filename); // Clean up
    }
}
