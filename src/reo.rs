use ring::digest::{Context, SHA256};
use ring::hmac;
use ring::signature::{EcdsaKeyPair, KeyPair, ECDSA_P256_SHA256_FIXED_SIGNING, ECDSA_P256_SHA256_FIXED};
use ring::rand::SystemRandom;
use ring::signature::{self, UnparsedPublicKey};
use std::error::Error;

// Hash data
pub fn whakamuna_raraunga(raraunga: &str) -> Result<String, Box<dyn Error>> {
    let mut horopaki = Context::new(&SHA256);
    horopaki.update(raraunga.as_bytes());
    let whakamuna = horopaki.finish();
    Ok(hex::encode(whakamuna.as_ref()))
}

<<<<<<< HEAD
// Create HMAC
=======
// Waihangahia te HMAC (Create HMAC)
>>>>>>> a719156a9e4dafdb76a81c779e6dcd594083b4be
pub fn hangaia_hmac(ki: &str, raraunga: &str) -> Result<String, Box<dyn Error>> {
    let hmac_ki = hmac::Key::new(hmac::HMAC_SHA256, ki.as_bytes());
    let waitohu = hmac::sign(&hmac_ki, raraunga.as_bytes());
    Ok(hex::encode(waitohu.as_ref()))
}

<<<<<<< HEAD
// Add a file
pub fn tapirihia_konae(ingoa: &str) -> Result<(), Box<dyn Error>> {
    let ara = Path::new(ingoa);
    File::create(&ara)?;
    println!("Konae '{}' kua tapirihia", ingoa);
    Ok(())
}

// Delete a file
pub fn mukua_konae(ingoa: &str) -> Result<(), Box<dyn Error>> {
    let ara = Path::new(ingoa);
    fs::remove_file(&ara)?;
    println!("Konae '{}' kua mukua", ingoa);
    Ok(())
}

// List files
pub fn rarangi_konae() -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        println!("{}", entry.file_name().into_string().unwrap());
    }
    Ok(())
}

pub struct ReoScript {
    pub waehere: String,
    pub params: Vec<String>,
    pub commands: HashMap<String, Box<dyn Fn(&[String]) -> Result<(), Box<dyn Error>>>>,
}

impl ReoScript {
    pub fn hou(waehere: &str, params: Vec<String>) -> Self {
        let mut commands: HashMap<String, Box<dyn Fn(&[String]) -> Result<(), Box<dyn Error>>>> = HashMap::new();

        commands.insert("whakamuna_raraunga".to_string(), Box::new(|params: &[String]| {
            if params.len() != 1 {
                return Err("Invalid number of parameters".into());
            }
            let hash = whakamuna_raraunga(&params[0])?;
            println!("Kua whakamunatia nga raraunga: {}", hash);
            Ok(())
        }));

        commands.insert("hangaia_hmac".to_string(), Box::new(|params: &[String]| {
            if params.len() != 2 {
                return Err("Invalid number of parameters".into());
            }
            let hmac = hangaia_hmac(&params[0], &params[1])?;
            println!("Kua hangaia te HMAC: {}", hmac);
            Ok(())
        }));

        commands.insert("tapirihia_konae".to_string(), Box::new(|params: &[String]| {
            if params.len() != 1 {
                return Err("Invalid number of parameters".into());
            }
            tapirihia_konae(&params[0])?;
            Ok(())
        }));

        commands.insert("mukua_konae".to_string(), Box::new(|params: &[String]| {
            if params.len() != 1 {
                return Err("Invalid number of parameters".into());
            }
            mukua_konae(&params[0])?;
            Ok(())
        }));

        commands.insert("rarangi_konae".to_string(), Box::new(|_params: &[String]| {
            rarangi_konae()?;
            Ok(())
        }));

        ReoScript {
            waehere: waehere.to_string(),
            params,
            commands,
        }
    }

    pub fn whakahaere(&self) {
        if let Some(command) = self.commands.get(&self.waehere) {
            if let Err(e) = command(&self.params) {
                println!("Hapa i te whakahaere whakahau: {}", e);
            }
        } else {
            println!("Kaore he mahi mo tenei waehere.");
        }
    }
                           }
=======
// Waihangahia te kīwaha matua (Create keypair)
pub fn hangaia_kiwaha_matua() -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
    let rng = SystemRandom::new();
    let pkcs8_bytes = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng)?;
    let kiwaha_matua = EcdsaKeyPair::from_pkcs8(&pkcs8_bytes)?;

    let ki_muna = pkcs8_bytes.as_ref().to_vec();
    let ku_tumatanui = kiwaha_matua.public_key().as_ref().to_vec();

    Ok((ki_muna, ki_tumatanui))
}

// Waitohua ngā raraunga (Sign data)
pub fn waitohua_raraunga(ki_muna: &[u8], raraunga: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let kiwaha_matua = EcdsaKeyPair::from_pkcs8(ki_muna)?;
    let rng = SystemRandom::new();
    let waitohu = kiwaha_matua.sign(&rng, raraunga)?;
    Ok(waitohu.as_ref().to_vec())
}

// Whakaūngia te waitohu (Verify signature)
pub fn whakau_waitohu(ki_tumatanui: &[u8], raraunga: &[u8], waitohu: &[u8]) -> Result<bool, Box<dyn Error>> {
    let ki_tumatanui = UnparsedPublicKey::new(&ECDSA_P256_SHA256_FIXED, ki_tumatanui);
    ki_tumatanui.verify(raraunga, waitohu).map_err(|_| "Waitohu kāore i te tika".into())
}
>>>>>>> a719156a9e4dafdb76a81c779e6dcd594083b4be
