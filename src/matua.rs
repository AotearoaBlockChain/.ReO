use ring::digest::{Context, SHA256};
use ring::hmac;
use ring::rand::{SecureRandom, SystemRandom};
use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::env;
use std::process;
use hex;
use warp::Filter;

mod nga_toi;
mod poutaka;
mod whatunga;
mod whakamatautau;
mod poraka;
mod whakahaere;
mod whakaaetanga;
mod puna;
mod reo;
mod munatanga;
mod koha;

// Error Handling
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

// Function to validate arguments
fn validate_args(args: &[String], expected: usize) -> Result<(), String> {
    if args.len() != expected {
        return Err(format!("Expected {} arguments, got {}", expected - 1, args.len() - 1));
    }
    Ok(())
}

// Main function to handle CLI arguments
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Kaore he whakahau i tukuna.");
        whakaatu_awhina();
        process::exit(1);
    }

    match args[1].as_str() {
        "panui_karere" => panui_karere(),
        "tapiiri_tau" => {
            validate_args(&args, 4).unwrap_or_else(|err| {
                eprintln!("Kaore i te tika: {}", err);
                process::exit(1);
            });
            let a = args[2].parse::<i32>().unwrap_or_else(|_| {
                eprintln!("Tau kaore i te tika: {}", args[2]);
                process::exit(1);
            });
            let b = args[3].parse::<i32>().unwrap_or_else(|_| {
                eprintln!("Tau kaore i te tika: {}", args[3]);
                process::exit(1);
            });
            tapiiri_tau(a, b);
        }
        "tango_tau" => {
            validate_args(&args, 4).unwrap_or_else(|err| {
                eprintln!("Kaore i te tika: {}", err);
                process::exit(1);
            });
            let a = args[2].parse::<i32>().unwrap_or_else(|_| {
                eprintln!("Tau kaore i te tika: {}", args[2]);
                process::exit(1);
            });
            let b = args[3].parse::<i32>().unwrap_or_else(|_| {
                eprintln!("Tau kaore i te tika: {}", args[3]);
                process::exit(1);
            });
            tango_tau(a, b);
        }
        "whakanuia_tau" => {
            validate_args(&args, 4).unwrap_or_else(|err| {
                eprintln!("Kaore i te tika: {}", err);
                process::exit(1);
            });
            let a = args[2].parse::<i32>().unwrap_or_else(|_| {
                eprintln!("Tau kaore i te tika: {}", args[2]);
                process::exit(1);
            });
            let b = args[3].parse::<i32>().unwrap_or_else(|_| {
                eprintln!("Tau kaore i te tika: {}", args[3]);
                process::exit(1);
            });
            whakanuia_tau(a, b);
        }
        "wehe_tau" => {
            validate_args(&args, 4).unwrap_or_else(|err| {
                eprintln!("Kaore i te tika: {}", err);
                process::exit(1);
            });
            let a = args[2].parse::<i32>().unwrap_or_else(|_| {
                eprintln!("Tau kaore i te tika: {}", args[2]);
                process::exit(1);
            });
            let b = args[3].parse::<i32>().unwrap_or_else(|_| {
                eprintln!("Tau kaore i te tika: {}", args[3]);
                process::exit(1);
            });
            wehe_tau(a, b);
        }
        "whakaatu_awhina" => whakaatu_awhina(),
        _ => {
            eprintln!("Whakahau kaore i te mohiotia: {}", args[1]);
            whakaatu_awhina();
            process::exit(1);
        }
    }
}

// Tests
#[cfg(test)]
mod whakamatautau {
    use super::*;

    #[test]
    fn test_waihanga_ki() {
        let key = waihanga_ki().expect("Failed to create key");
        assert_eq!(key.len(), 64); // 32 bytes hex-encoded
    }

    // Add more tests as needed...
}