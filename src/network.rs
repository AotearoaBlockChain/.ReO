// src/network.rs

use warp::Filter;
use serde::{Deserialize, Serialize};
use log::info;
use warp::reject::Reject;
use warp::http::StatusCode;

#[derive(Debug, Deserialize, Serialize)]
struct TauhoheApi {
    hua: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct UrungaRaraunga {
    field1: String,
    field2: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct UrungaHmac {
    // Add relevant fields
}

#[derive(Debug, Deserialize, Serialize)]
struct UrungaWhakamuna {
    // Add relevant fields
}

#[derive(Debug, Deserialize, Serialize)]
struct UrungaWetekina {
    // Add relevant fields
}

// Custom error type to implement `Reject` trait for warp rejection handling
#[derive(Debug)]
struct CustomError {
    message: String,
}

impl Reject for CustomError {}

async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(_) = err.find::<warp::reject::InvalidQuery>() {
        Ok(warp::reply::with_status(
            "Invalid query parameters".to_string(),
            StatusCode::BAD_REQUEST,
        ))
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        Ok(warp::reply::with_status(
            "Method not allowed".to_string(),
            StatusCode::METHOD_NOT_ALLOWED,
        ))
    } else if let Some(custom_error) = err.find::<CustomError>() {
        Ok(warp::reply::with_status(
            custom_error.message.clone(),
            StatusCode::BAD_REQUEST,
        ))
    } else {
        // For any other errors, return a 500 Internal Server Error
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}

#[allow(dead_code)]
pub async fn run_server() {
    let aratuka_whakamuka = warp::path("whakamuka")
        .and(warp::body::json())
        .map(|body: UrungaRaraunga| {
            info!("Received data at /whakamuka: {:?}", body);
            warp::reply::json(&TauhoheApi {
                hua: format!("Received: {:?}", body),

            });

#[allow(dead_code)]
pub async fn run_server() {
    // Your code here
}
        

            });
        })
        .recover(handle_rejection);  // Add error recovery
}

    let aratuka_hmac = warp::path("hmac")
        .and(warp::body::json())
        .map(|body: UrungaHmac| {
            info!("Received HMAC at /hmac: {:?}", body);
            warp::reply::json(&TauhoheApi {
                hua: format!("Received HMAC: {:?}", body),
            })
        })
        .recover(handle_rejection);  // Add error recovery

    let aratuka_hanga_ki = warp::path("hanga_ki")
        .and(warp::body::json())
        .map(|body: UrungaWhakamuna| {
            info!("Received Whakamuna at /hanga_ki: {:?}", body);
            warp::reply::json(&TauhoheApi {
                hua: format!("Received Whakamuna: {:?}", body),
            })
        })
        .recover(handle_rejection);  // Add error recovery

    let aratuka_whakamuna = warp::path("whakamuna")
        .and(warp::body::json())
        .map(|body: UrungaWetekina| {
            info!("Received Wetekina at /whakamuna: {:?}", body);
            warp::reply::json(&TauhoheApi {
                hua: format!("Received Wetekina: {:?}", body),
            })
        })
        .recover(handle_rejection);  // Add error recovery

    // Combine all routes ensuring they all return a valid Reply
    let aratuka = aratuka_whakamuka
        .or(aratuka_hmac)
        .or(aratuka_hanga_ki)
        .or(aratuka_whakamuna)
        .recover(handle_rejection);  // Add global error recovery

    // Start the server with all routes
    info!("API Server running at http://127.0.0.1:8080");
    warp::serve(aratuka).run(([127, 0, 0, 1], 8080)).await;
}
