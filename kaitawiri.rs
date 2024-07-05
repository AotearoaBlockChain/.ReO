use crate::{Mana, MomoMana};

pub fn kaitawiri(tumau_tangata: &str) -> Vec<Mana> {
    // Implementation for tokenization
    // Replace with actual tokenization logic
    let tokens: Vec<&str> = tumau_tangata.split_whitespace().collect();
    let mut mana_vec = Vec::new();

    for token in tokens {
        let momo_mana = match token {
            "TohuTangata" => MomoMana::TohuTangata,
            "ManaTohu" => MomoMana::ManaTohu,
            "Koohatu" => MomoMana::Koohatu,
            "Tohu" => MomoMana::Tohu,
            _ => {
                println!("Unknown token: {}", token);
                continue;
            }
        };

        let mana = Mana {
            momo_mana,
            uara: token.to_string(),
            raina: 1,  // Example value
        };

        mana_vec.push(mana);
    }

    mana_vec
          }
