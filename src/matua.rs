use ring::digest::{Context, SHA256};
use ring::hmac;
use ring::rand::{SecureRandom, SystemRandom};
use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
use std::error::Error;
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use hex;
use warp::Filter;

#[cfg(test)]
mod whakamatautau;
mod whatunga;

#[tokio::main]
async fn main() {
    // GET /Kia Ora => 200 OK with body "Kia Ora, Aotearoa!"

    let hello = warp::path("Kia Ora")
        .map(|| "Kia Ora, Aotearoa!");

    // GET / => 200 OK with body "Warp server is running!"
    let root = warp::path::end()
        .map(|| "Warp server is running!");

    // Combine the routes
    let routes = hello.or(root);

    // Start the server on port 8080
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;

    let read_data: &[u8] = &[1, 2, 3, 4];

    let binary_data: &[u8] = &[1, 2, 3, 4];

    // Use assert_eq! to compare the two byte slices
    assert_eq!(read_data, binary_data);

}

#[derive(Debug)]
pub enum ReOError {
    IoError(io::Error),
    RingError(ring::error::Unspecified),
    HexError(hex::FromHexError),
}

impl fmt::Display for ReOError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ReOError::IoError(ref err) => write!(f, "IO error: {}", err),
            ReOError::RingError(ref err) => write!(f, "Ring error: {:?}", err),
            ReOError::HexError(ref err) => write!(f, "Hex error: {:?}", err),
        }
    }
}

impl Error for ReOError {}

impl From<io::Error> for ReOError {
    fn from(err: io::Error) -> ReOError {
        ReOError::IoError(err)
    }
}

impl From<ring::error::Unspecified> for ReOError {
    fn from(err: ring::error::Unspecified) -> ReOError {
        ReOError::RingError(err)
    }
}

impl From<hex::FromHexError> for ReOError {
    fn from(err: hex::FromHexError) -> ReOError {
        ReOError::HexError(err)
    }
}

pub fn whakamuka(raraunga: &str) -> Result<String, ReOError> {
    let mut horopaki = Context::new(&SHA256);
    horopaki.update(raraunga.as_bytes());
    let whakamuka = horopaki.finish();
    Ok(hex::encode(whakamuka.as_ref()))
}

pub fn hangaia_hmac(ki: &str, raraunga: &str) -> Result<String, ReOError> {
    let hmac_ki = hmac::Key::new(hmac::HMAC_SHA256, ki.as_bytes());
    let waitohu = hmac::sign(&hmac_ki, raraunga.as_bytes());
    Ok(hex::encode(waitohu.as_ref()))
}

pub fn waihanga_ki() -> Result<String, ReOError> {
    let rng = SystemRandom::new();
    let mut ki = [0u8; 32];
    rng.fill(&mut ki)?;
    Ok(hex::encode(ki))
}

pub fn whakamuna_raraunga_aead(ki_hex: &str, raraunga: &[u8]) -> Result<(Vec<u8>, Vec<u8>), ReOError> {
    let ki = hex::decode(ki_hex)?;

    if ki.len() != 32 {
        return Err(ReOError::RingError(ring::error::Unspecified));
    }

    let ki_matapokere = UnboundKey::new(&AES_256_GCM, &ki)?;
    let nonce_purua = {
        let rng = SystemRandom::new();
        let mut nonce = [0u8; 12];
        rng.fill(&mut nonce)?;
        nonce
    };
    let nonce = Nonce::assume_unique_for_key(nonce_purua);
    let mut in_out = raraunga.to_vec();
    let ki_powhiri = LessSafeKey::new(ki_matapokere);

    match ki_powhiri.seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out) {
        Ok(()) => Ok((nonce_purua.to_vec(), in_out)),
        Err(e) => Err(ReOError::RingError(e))
    }
}

pub fn wetekina_raraunga_aead(ki_hex: &str, nonce: &[u8], whakamuna: &[u8]) -> Result<Vec<u8>, ReOError> {
    let ki = hex::decode(ki_hex)?;

    if ki.len() != 32 {
        return Err(ReOError::RingError(ring::error::Unspecified));
    }

    let ki_matapokere = UnboundKey::new(&AES_256_GCM, &ki)?;
    let nonce = Nonce::try_assume_unique_for_key(nonce)?;
    let mut in_out = whakamuna.to_vec();
    let ki_powhiri = LessSafeKey::new(ki_matapokere);

    match ki_powhiri.open_in_place(nonce, Aad::empty(), &mut in_out) {
        Ok(data) => Ok(data.to_vec()),
        Err(e) => Err(ReOError::RingError(e))
    }
}

pub fn tapirihia_konae(ingoa_konae: &str) -> Result<(), ReOError> {
    let ara = Path::new(ingoa_konae);
    if ara.exists() {
        return Err(ReOError::IoError(io::Error::new(io::ErrorKind::AlreadyExists, "File already exists")));
    }
    File::create(&ara)?;
    Ok(())
}

pub fn mukua_konae(ingoa_konae: &str) -> Result<(), ReOError> {
    let ara = Path::new(ingoa_konae);
    fs::remove_file(&ara)?;

    // Retry mechanism to ensure the file is deleted
    for _ in 0..5 {
        if !ara.exists() {
            return Ok(());
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Err(ReOError::IoError(io::Error::new(io::ErrorKind::Other, "Failed to delete file")))
}

pub fn panuihia_konae(ingoa_konae: &str) -> Result<Vec<u8>, ReOError> {
    let mut ara = File::open(ingoa_konae)?;
    let mut ihirangi = Vec::new();
    ara.read_to_end(&mut ihirangi)?;
    Ok(ihirangi)
}

pub fn tapirihia_raraunga(ingoa_konae: &str, raraunga: &str) -> Result<(), ReOError> {
    let mut ara = OpenOptions::new().append(true).open(ingoa_konae)?;
    ara.write_all(raraunga.as_bytes())?;

    Ok(())
}
