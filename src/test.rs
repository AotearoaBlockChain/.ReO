use std::io::Write;
use std::io::Read;

use crate::{whakamuka, hangaia_hmac, tapirihia_konae, mukua_konae, panuihia_konae, tapirihia_raraunga};

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
fn test_mukua_konae() {
    let ingoa_konae = "testfile.txt";
    // Ensure the file is created
    let _ = File::create(ingoa_konae).expect("Failed to create file");
    assert!(Path::new(ingoa_konae).exists(), "File should exist before deletion");

    // Attempt to delete the file
    let result = mukua_konae(ingoa_konae);
    assert!(result.is_ok(), "Failed to delete file: {:?}", result);
    assert!(!Path::new(ingoa_konae).exists(), "File still exists after deletion");
}

    #[test]
    fn test_mukua_konae() {
        // Setup code here...

        // Perform the file deletion
        std::fs::remove_file("path/to/file").expect("Failed to delete file");

        // Add a short delay to ensure the filesystem has completed the deletion
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Check that the file has been deleted
        assert!(!std::fs::metadata("path/to/file").is_ok(), "File still exists after deletion");
    }

    // Add other tests here...
}
