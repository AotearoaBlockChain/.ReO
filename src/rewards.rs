// rewards.rs

pub fn tohatoha_utunga(tangata: &[(String, u64, u64)]) -> Result<(), String> {
    for (tangata_id, kaute_whaiwahi, rahinga_punga) in tangata {
        if *kaute_whaiwahi == 0 || *rahinga_punga == 0 {
            return Err(format!("Invalid values for {}: kaute_whaiwahi={}, rahinga_punga={}", tangata_id, kaute_whaiwahi, rahinga_punga));
        }

        let utunga = tatau_utunga(kaute_whaiwahi, rahinga_punga);
        log::info!("Tohatoha ki a {} me te utunga {}", tangata_id, utunga);
        // Whakauru i te tikanga tohatoha utunga ki konei
    }
    Ok(())
}

fn tatau_utunga(kaute_whaiwahi: u64, rahinga_punga: u64) -> u64 {
    kaute_whaiwahi + rahinga_punga // Tauira tatau utunga
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tohatoha_utunga() {
        let tangata = vec![
            ("Tangata1".to_string(), 100, 50),
            ("Tangata2".to_string(), 200, 75),
        ];
        let result = tohatoha_utunga(&tangata);
        assert!(result.is_ok());
    }

    #[test]
    fn test_tatau_utunga() {
        assert_eq!(tatau_utunga(100, 50), 150);
        assert_eq!(tatau_utunga(200, 75), 275);
    }

    #[test]
    fn test_tohatoha_utunga_invalid() {
        let tangata = vec![
            ("Tangata1".to_string(), 0, 50),
            ("Tangata2".to_string(), 200, 0),
        ];
        let result = tohatoha_utunga(&tangata);
        assert!(result.is_err());
    }
}