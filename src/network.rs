use warp::Filter;
use ring::digest::{Context, SHA256};
use ring::hmac;
use ring::rand::{SecureRandom, SystemRandom};
use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
use hex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct UrungaRaraunga {
    raraunga: String,
}

#[derive(Debug, Deserialize)]
struct UrungaHmac {
    ki: String,
    raraunga: String,
}

#[derive(Debug, Serialize)]
struct TauhoheApi<T> {
    hua: T,
}

#[derive(Debug, Deserialize)]
struct UrungaWhakamuna {
    ki: String,
    raraunga: String,
}

#[derive(Debug, Deserialize)]
struct UrungaWetekina {
    ki: String,
    nonce: String,
    whakamuna: String,
}

pub async fn whakahaere_tumau() {
    let aratuka_whakamuka = warp::path!("whakamuka")
        .and(warp::post())
        .and(warp::body::json())
        .map(|urunga: UrungaRaraunga| {
            let mut horopaki = Context::new(&SHA256);
            horopaki.update(urunga.raraunga.as_bytes());
            let whakamuka = horopaki.finish();
            warp::reply::json(&TauhoheApi { hua: hex::encode(whakamuka.as_ref()) })
        });

    let aratuka_hmac = warp::path!("hmac")
        .and(warp::post())
        .and(warp::body::json())
        .map(|urunga: UrungaHmac| {
            let ki = hmac::Key::new(hmac::HMAC_SHA256, urunga.ki.as_bytes());
            let waitohu = hmac::sign(&ki, urunga.raraunga.as_bytes());
            warp::reply::json(&TauhoheApi { hua: hex::encode(waitohu.as_ref()) })
        });

    let aratuka_hanga_ki = warp::path!("ki")
        .and(warp::post())
        .map(|| {
            let rng = SystemRandom::new();
            let mut ki = [0u8; 32];
            rng.fill(&mut ki).unwrap();
            warp::reply::json(&TauhoheApi { hua: hex::encode(ki) })
        });

    let aratuka_whakamuna = warp::path!("whakamuna")
        .and(warp::post())
        .and(warp::body::json())
        .map(|urunga: UrungaWhakamuna| {
            let ki = hex::decode(urunga.ki).unwrap();
            let raraunga = urunga.raraunga.as_bytes();
            let mut rng = SystemRandom::new();
            let mut nonce = [0u8; 12];
            rng.fill(&mut nonce).unwrap();

            let unbound_key = UnboundKey::new(&AES_256_GCM, &ki).unwrap();
            let less_safe_key = LessSafeKey::new(unbound_key);
            let nonce = Nonce::assume_unique_for_key(nonce);
            let mut in_out = raraunga.to_vec();
            less_safe_key
                .seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
                .unwrap();

            warp::reply::json(&TauhoheApi {
                hua: format!(
                    "{{\"nonce\": \"{}\", \"whakamuna\": \"{}\"}}",
                    hex::encode(nonce.as_ref()),
                    hex::encode(&in_out)
                ),
            })
        });

    let aratuka_wetekina = warp::path!("wetekina")
        .and(warp::post())
        .and(warp::body::json())
        .map(|urunga: UrungaWetekina| {
            let ki = hex::decode(urunga.ki).unwrap();
            let nonce = hex::decode(urunga.nonce).unwrap();
            let whakamuna = hex::decode(urunga.whakamuna).unwrap();

            let unbound_key = UnboundKey::new(&AES_256_GCM, &ki).unwrap();
            let less_safe_key = LessSafeKey::new(unbound_key);
            let nonce = Nonce::try_assume_unique_for_key(&nonce).unwrap();
            let mut in_out = whakamuna.to_vec();
            let raraunga_wetekina = less_safe_key
                .open_in_place(nonce, Aad::empty(), &mut in_out)
                .unwrap();

            warp::reply::json(&TauhoheApi {
                hua: String::from_utf8(raraunga_wetekina.to_vec()).unwrap(),
            })
        });

    let aratuka = aratuka_whakamuka.or(aratuka_hmac).or(aratuka_hanga_ki).or(aratuka_whakamuna).or(aratuka_wetekina);

    warp::serve(aratuka).run(([127, 0, 0, 1], 3030)).await;
}
