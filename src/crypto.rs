use ring::digest::{Context, Digest, SHA256};
use ring::hmac;

// Whakamunatia ngā raraunga
pub fn whakamuna_raraunga(data: &str) -> String {
    let mut context = Context::new(&SHA256);
    context.update(data.as_bytes());
    let digest = context.finish();
    hex::encode(digest.as_ref())
}

// Waihangahia te HMAC
pub fn hangaia_hmac(ki: &str, raraunga: &str) -> String {
    let hmac_ki = hmac::Key::new(hmac::HMAC_SHA256, ki.as_bytes());
    let waitohu = hmac::sign(&hmac_ki, raraunga.as_bytes());
    hex::encode(waitohu.as_ref())
}
