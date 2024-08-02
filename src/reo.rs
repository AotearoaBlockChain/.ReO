use ring::digest::{Context, Digest, SHA256};
use ring::hmac;
use ring::signature::{EcdsaKeyPair, KeyPair, ECDSA_P256_SHA256_FIXED_SIGNING, ECDSA_P256_SHA256_FIXED};
use ring::rand::SystemRandom;
use ring::signature::{self, UnparsedPublicKey};
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

// Waihangahia te kīwaha matua (Create keypair)
pub fn hangaia_kīwaha_matua() -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
    let rng = SystemRandom::new();
    let pkcs8_bytes = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng)?;
    let kīwaha_matua = EcdsaKeyPair::from_pkcs8(&pkcs8_bytes)?;

    let kī_muna = pkcs8_bytes.as_ref().to_vec();
    let kī_tūmatanui = kīwaha_matua.public_key().as_ref().to_vec();

    Ok((kī_muna, kī_tūmatanui))
}

// Waitohua ngā raraunga (Sign data)
pub fn waitohua_raraunga(kī_muna: &[u8], raraunga: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let kīwaha_matua = EcdsaKeyPair::from_pkcs8(kī_muna)?;
    let rng = SystemRandom::new();
    let waitohu = kīwaha_matua.sign(&rng, raraunga)?;
    Ok(waitohu.as_ref().to_vec())
}

// Whakaūngia te waitohu (Verify signature)
pub fn whakaū_waitohu(kī_tūmatanui: &[u8], raraunga: &[u8], waitohu: &[u8]) -> Result<bool, Box<dyn Error>> {
    let kī_tūmatanui = UnparsedPublicKey::new(&ECDSA_P256_SHA256_FIXED, kī_tūmatanui);
    kī_tūmatanui.verify(raraunga, waitohu).map_err(|_| "Waitohu kāore i te tika".into())
}
