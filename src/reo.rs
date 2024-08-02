use ring::digest::{Context, Digest, SHA256};
use ring::hmac;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::collections::HashMap;
use std::error::Error;

// Whakamunatia ngā raraunga (Hash data)
pub fn whakamuna_raraunga(raraunga: &str) -> Result<String, Box<dyn Error>> {
    let mut horopaki = Context::new(&SHA256);
    horopaki.update(raraunga.as_bytes());
    let whakamuna = horopaki.finish();
    Ok(hex::encode(whakamuna.as_ref()))
}

// Waihangahia te HMAC (Create HMAC)
pub fn hangaia_hmac(kī: &str, raraunga: &str) -> Result<String, Box<dyn Error>> {
    let hmac_kī = hmac::Key::new(hmac::HMAC_SHA256, kī.as_bytes());
    let waitohu = hmac::sign(&hmac_kī, raraunga.as_bytes());
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

// Rārangi ngā konae (List files)
pub fn rarangi_konae() -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        println!("{}", entry.file_name().into_string().unwrap());
    }
    Ok(())
}

pub struct ReoScript {
    pub waehere: String,
    pub commands: HashMap<String, Box<dyn Fn() -> Result<(), Box<dyn Error>>>>,
}

impl ReoScript {
    pub fn hou(waehere: &str) -> Self {
        let mut commands: HashMap<String, Box<dyn Fn() -> Result<(), Box<dyn Error>>>> = HashMap::new();

        commands.insert("whakamuna_raraunga".to_string(), Box::new(|| {
            let raraunga = "ētahi raraunga hei whakamuna";
            let hash = whakamuna_raraunga(raraunga)?;
            println!("Kua whakamunatia ngā raraunga: {}", hash);
            Ok(())
        }));

        commands.insert("hangaia_hmac".to_string(), Box::new(|| {
            let kī = "ki_muna";
            let raraunga = "ētahi raraunga hei waitohu";
            let hmac = hangaia_hmac(kī, raraunga)?;
            println!("Kua hangaia te HMAC: {}", hmac);
            Ok(())
        }));

        commands.insert("tapirihia_konae".to_string(), Box::new(|| {
            let ingoa = "tauira.txt";
            tapirihia_konae(ingoa)?;
            Ok(())
        }));

        commands.insert("mukua_konae".to_string(), Box::new(|| {
            let ingoa = "tauira.txt";
            mukua_konae(ingoa)?;
            Ok(())
        }));

        commands.insert("rarangi_konae".to_string(), Box::new(|| {
            rarangi_konae()?;
            Ok(())
        }));

        ReoScript {
            waehere: waehere.to_string(),
            commands,
        }
    }

    pub fn whakahaere(&self) {
        if let Some(command) = self.commands.get(&self.waehere) {
            if let Err(e) = command() {
                println!("Hapa i te whakahaere whakahau: {}", e);
            }
        } else {
            println!("Kāore he mahi mō tēnei waehere.");
        }
    }
        }
