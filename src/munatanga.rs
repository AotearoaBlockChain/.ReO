extern crate ring;
use ring::digest::{Context, SHA256};
use ring::hmac;
use ring::signature::{EcdsaKeyPair, KeyPair, ECDSA_P256_SHA256_FIXED_SIGNING, ECDSA_P256_SHA256_FIXED};
use ring::rand::SystemRandom;
use ring::signature::UnparsedPublicKey;
use std::error::Error;

// Whakamunatia nga raraunga (Hash data)
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

// Waihangahia te kiwaha matua (Create keypair)
pub fn hangaia_kiwaha_matua() -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
    let rng = SystemRandom::new();
    let pkcs8_bytes = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng)
        .map_err(|_| "I rahua te waihanga i te kiwaha matua")?;
    let pkcs8_bytes_ref = pkcs8_bytes.as_ref();
    let kiwaha_matua = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8_bytes_ref)
        .map_err(|_| "I rahua te waihanga i te kiwaha matua mai i te pkcs8")?;

    let ki_muna = pkcs8_bytes_ref.to_vec();
    let ki_tumatanui = kiwaha_matua.public_key().as_ref().to_vec();

    Ok((ki_muna, ki_tumatanui))
}

// Waitohua nga raraunga (Sign data)
pub fn waitohua_raraunga(ki_muna: &[u8], raraunga: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let kiwaha_matua = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, ki_muna)
        .map_err(|_| "I rahua te waihanga i te kiwaha matua mai i te pkcs8")?;
    let rng = SystemRandom::new();
    let waitohu = kiwaha_matua.sign(&rng, raraunga)
        .map_err(|_| "I rahua te waitohu i nga raraunga")?;
    Ok(waitohu.as_ref().to_vec())
}

// Whakaungia te waitohu (Verify signature)
pub fn whakau_waitohu(ki_tumatanui: &[u8], raraunga: &[u8], waitohu: &[u8]) -> Result<bool, Box<dyn Error>> {
    let ki_tumatanui = UnparsedPublicKey::new(&ECDSA_P256_SHA256_FIXED, ki_tumatanui);
    ki_tumatanui.verify(raraunga, waitohu).map(|_| true).map_err(|_| "Waitohu kaore i te tika".into())
}
