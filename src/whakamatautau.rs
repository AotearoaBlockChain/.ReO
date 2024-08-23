use crate::whakamuka;
use crate::hangaia_hmac;
use crate::tapirihia_komae;
use crate::tapirihia_raraunga;
use crate::mukua_konae;
use crate::panuhia_konae;

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;
    use std::io::read;

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
    fn test_whakamuka_long_input() {
        let raraunga = "a".repeat(1000000); // 1 million characters
        let result = whakamuka(&raraunga);
        assert!(result.is_ok());
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
    fn test_hangaia_hmac_special_characters() {
        let raraunga = "Hello, world! @#$%^&*()";
        let ki = "supersecretkey!@#$%^&*()";
        let result = hangaia_hmac(ki, raraunga);
        assert!(result.is_ok());
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
    fn test_tapirihia_raraunga_multiple_appends() {
        let ingoa_konae = "testfile.txt";
        let _ = File::create(ingoa_konae);
        let raraunga1 = "This is some test data.";
        let raraunga2 = " Additional data.";
        let raraunga3 = " Even more data.";
        
        tapirihia_raraunga(ingoa_konae, raraunga1).unwrap();
        tapirihia_raraunga(ingoa_konae, raraunga2).unwrap();
        tapirihia_raraunga(ingoa_konae, raraunga3).unwrap();
        
        let mut file_content = String::new();
        let mut file = File::open(ingoa_konae).unwrap();
        file.read_to_string(&mut file_content).unwrap();
        
        assert!(file_content.contains(raraunga1));
        assert!(file_content.contains(raraunga2));
        assert!(file_content.contains(raraunga3));
        
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

        // Retry logic to check if the file has been deleted, with a short delay
        for _ in 0..5 {
            if !Path::new(ingoa_konae).exists() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        // Final check
        assert!(!Path::new(ingoa_konae).exists(), "File still exists after deletion");
    }

    #[test]
    fn test_mukua_konae_nonexistent() {
        let ingoa_konae = "nonexistent.txt";
        let result = mukua_konae(ingoa_konae);
        assert!(result.is_err()); // Expect an error because the file does not exist
    }

    #[test]
    fn test_mukua_konae_concurrent_access() {
        let ingoa_konae = "testfile.txt";
        let _ = File::create(ingoa_konae).expect("Failed to create file");
        assert!(Path::new(ingoa_konae).exists(), "File should exist before deletion");

        let handle = thread::spawn(move || {
            mukua_konae(ingoa_konae).unwrap();
        });

        // Retry mechanism: Give a small delay before trying to access the file.
        std::thread::sleep(std::time::Duration::from_millis(50));

        // Simulate concurrent access by attempting to open the file during deletion
        let open_result = File::open(ingoa_konae);
        assert!(open_result.is_err(), "File should not be accessible during deletion");

        handle.join().expect("Thread failed");

        // Final check
        assert!(!Path::new(ingoa_konae).exists(), "File still exists after deletion");
    }

    #[test]
    fn test_panuihia_konae_binary_file() {
        let ingoa_konae = "testfile.bin";
        let binary_data = vec![0, 159, 146, 150];
        let mut file = File::create(ingoa_konae).unwrap();
        file.write_all(&binary_data).unwrap();
        
        let result = panuihia_konae(ingoa_konae);
        assert!(result.is_ok());
        let read_data = result.unwrap();
        assert_eq!(read_data.into_bytes(), binary_data);
        assert_eq!(read_data, binary_data);  // Compare directly as Vec<u8>
        
        let _ = fs::remove_file(ingoa_konae);
    }

    #[test]
    fn test_panuihia_konae_nonexistent() {
        let ingoa_konae = "nonexistent.txt";
        let result = panuihia_konae(ingoa_konae);
        assert!(result.is_err()); // Expect an error because the file does not exist
    }

    fn test_panuihia_konae() {
        let ingoa_konae = "testfile.txt";
        let content = "This is a test file content.";
        let mut file = File::create(ingoa_konae).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        let result = panuihia_konae(ingoa_konae);
        assert!(result.is_ok());

        // Convert read_data (Vec<u8>) back to String for comparison
        let read_data = String::from_utf8(result.unwrap()).unwrap();
        assert_eq!(read_data, content);  // Compare as Strings

        let _ = fs::remove_file(ingoa_konae);
    }
}
