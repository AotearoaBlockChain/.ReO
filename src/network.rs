// src/network.rs

use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Debug, Deserialize, Serialize)]
struct TauhoheApi {
    hua: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct UrungaRaraunga {
    field1: String,
    field2: String,
}

// Add more structs as necessary
#[derive(Debug, Deserialize, Serialize)]
struct UrungaHmac {
    // fields
}

#[derive(Debug, Deserialize, Serialize)]
struct UrungaWhakamuna {
    // fields
}

#[derive(Debug, Deserialize, Serialize)]
struct UrungaWetekina {
    // fields
}

#[tokio::main]
async fn main() {
    let aratuka_whakamuka = warp::path("whakamuka")
        .and(warp::body::json())
        .map(|body: UrungaRaraunga| {
            warp::reply::json(&TauhoheApi {
                hua: format!("Received: {:?}", body),
            })
        });

    let aratuka_hmac = warp::path("hmac")
        .and(warp::body::json())
        .map(|body: UrungaHmac| {
            warp::reply::json(&TauhoheApi {
                hua: format!("Received HMAC: {:?}", body),
            })
        });

    let aratuka_hanga_ki = warp::path("hanga_ki")
        .and(warp::body::json())
        .map(|body: UrungaWhakamuna| {
            warp::reply::json(&TauhoheApi {
                hua: format!("Received Whakamuna: {:?}", body),
            })
        });

    let aratuka_whakamuna = warp::path("whakamuna")
        .and(warp::body::json())
        .map(|body: UrungaWetekina| {
            warp::reply::json(&TauhoheApi {
                hua: format!("Received Wetekina: {:?}", body),
            })
        });

    let aratuka = aratuka_whakamuka
        .or(aratuka_hmac)
        .or(aratuka_hanga_ki)
        .or(aratuka_whakamuna);

    warp::serve(aratuka).run(([127, 0, 0, 1], 3030)).await;
}
