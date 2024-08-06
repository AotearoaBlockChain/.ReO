use warp::Filter;
use ring::digest::{Context, SHA256};
use ring::hmac;
use ring::rand::{SecureRandom, SystemRandom};
use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
use hex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct DataInput {
    data: String,
}

#[derive(Debug, Deserialize)]
struct HmacInput {
    key: String,
    data: String,
}

#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    result: T,
}

#[derive(Debug, Deserialize)]
struct EncryptInput {
    key: String,
    data: String,
}

#[derive(Debug, Deserialize)]
struct DecryptInput {
    key: String,
    nonce: String,
    encrypted: String,
}

pub async fn run_server() {
    let hash_route = warp::path!("hash")
        .and(warp::post())
        .and(warp::body::json())
        .map(|input: DataInput| {
            let mut context = Context::new(&SHA256);
            context.update(input.data.as_bytes());
            let hash = context.finish();
            warp::reply::json(&ApiResponse { result: hex::encode(hash.as_ref()) })
        });

    let hmac_route = warp::path!("hmac")
        .and(warp::post())
        .and(warp::body::json())
        .map(|input: HmacInput| {
            let key = hmac::Key::new(hmac::HMAC_SHA256, input.key.as_bytes());
            let signature = hmac::sign(&key, input.data.as_bytes());
            warp::reply::json(&ApiResponse { result: hex::encode(signature.as_ref()) })
        });

    let key_route = warp::path!("key")
        .and(warp::post())
        .map(|| {
            let rng = SystemRandom::new();
            let mut key = [0u8; 32];
            rng.fill(&mut key).unwrap();
            warp::reply::json(&ApiResponse { result: hex::encode(key) })
        });

    let encrypt_route = warp::path!("encrypt")
        .and(warp::post())
        .and(warp::body::json())
        .map(|input: EncryptInput| {
            let key = hex::decode(input.key).unwrap();
            let data = input.data.as_bytes();
            let mut rng = SystemRandom::new();
            let mut nonce = [0u8; 12];
            rng.fill(&mut nonce).unwrap();

            let unbound_key = UnboundKey::new(&AES_256_GCM, &key).unwrap();
            let less_safe_key = LessSafeKey::new(unbound_key);
            let nonce = Nonce::assume_unique_for_key(nonce);
            let mut in_out = data.to_vec();
            less_safe_key
                .seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
                .unwrap();

            warp::reply::json(&ApiResponse {
                result: format!(
                    "{{\"nonce\": \"{}\", \"encrypted\": \"{}\"}}",
                    hex::encode(nonce.as_ref()),
                    hex::encode(&in_out)
                ),
            })
        });

    let decrypt_route = warp::path!("decrypt")
        .and(warp::post())
        .and(warp::body::json())
        .map(|input: DecryptInput| {
            let key = hex::decode(input.key).unwrap();
            let nonce = hex::decode(input.nonce).unwrap();
            let encrypted = hex::decode(input.encrypted).unwrap();

            let unbound_key = UnboundKey::new(&AES_256_GCM, &key).unwrap();
            let less_safe_key = LessSafeKey::new(unbound_key);
            let nonce = Nonce::try_assume_unique_for_key(&nonce).unwrap();
            let mut in_out = encrypted.to_vec();
            let decrypted_data = less_safe_key
                .open_in_place(nonce, Aad::empty(), &mut in_out)
                .unwrap();

            warp::reply::json(&ApiResponse {
                result: String::from_utf8(decrypted_data.to_vec()).unwrap(),
            })
        });

    let routes = hash_route.or(hmac_route).or(key_route).or(encrypt_route).or(decrypt_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
             }
