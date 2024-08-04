use ring::digest::{Context, SHA256};
use ring::hmac;
use ring::signature::{EcdsaKeyPair, KeyPair, ECDSA_P256_SHA256_FIXED_SIGNING};
use ring::rand::SystemRandom;
use ring::signature::{self, UnparsedPublicKey};
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
