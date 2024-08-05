// rewards.rs

pub fn tohatoha_utunga(tangata: &[(String, u64, u64)]) {
    for (tangata_id, kaute_whaiwahi, rahinga_punga) in tangata {
        let utunga = tatau_utunga(kaute_whaiwahi, rahinga_punga);
        println!("Tohatoha ki a {} me te utunga {}", tangata_id, utunga);
        // Whakauru i te tikanga tohatoha utunga ki konei
    }
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
        tohatoha_utunga(&tangata);
    }

    #[test]
    fn test_tatau_utunga() {
        assert_eq!(tatau_utunga(100, 50), 150);
        assert_eq!(tatau_utunga(200, 75), 275);
    }
}
