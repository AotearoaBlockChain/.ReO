pub struct Poraka {
    pub taupanga: u64,
    pub wa_timestamp: u128,
    pub hash_o_mua: String,
    pub hash: String,
    pub raraunga: String,
    pub nonce: u64,
}

impl Poraka {
    pub fn hou(taupanga: u64, wa_timestamp: u128, hash_o_mua: String, raraunga: String) -> Self {
        let nonce = 0;
        let hash = Self::tatauria_hash(taupanga, wa_timestamp, &hash_o_mua, &raraunga, nonce);
        Poraka {
            taupanga,
            wa_timestamp,
            hash_o_mua,
            hash,
            raraunga,
            nonce,
        }
    }

    pub fn tatauria_hash(taupanga: u64, wa_timestamp: u128, hash_o_mua: &str, raraunga: &str, nonce: u64) -> String {
        // Implement hash calculation using cryptographic function
        format!("{}_{}_{}_{}_{}", taupanga, wa_timestamp, hash_o_mua, raraunga, nonce) // Example placeholder
    }

    pub fn maina_poraka(&mut self, uaua: usize) {
        while &self.hash[..uaua] != "0".repeat(uaua) {
            self.nonce += 1;
            self.hash = Self::tatauria_hash(self.taupanga, self.wa_timestamp, &self.hash_o_mua, &self.raraunga, self.nonce);
        }
    }
}

pub struct WhatungaPoraka {
    pub mekameka: Vec<Poraka>,
    pub uaua: usize,
}

impl WhatungaPoraka {
    pub fn hou() -> Self {
        let poraka_kaumatua = Poraka::hou(0, wa_o_naianei(), String::from("0"), String::from("Poraka Kaumatua"));
        WhatungaPoraka {
            mekameka: vec![poraka_kaumatua],
            uaua: 4,
        }
    }

    pub fn tapiri_poraka(&mut self, raraunga: String) {
        let poraka_o_mua = self.mekameka.last().unwrap();
        let mut poraka_hou = Poraka::hou(
            poraka_o_mua.taupanga + 1,
            wa_o_naianei(),
            poraka_o_mua.hash.clone(),
            raraunga,
        );
        poraka_hou.maina_poraka(self.uaua);
        self.mekameka.push(poraka_hou);
    }

    pub fn he_tika_te_mekameka(&self) -> bool {
        for i in 1..self.mekameka.len() {
            let poraka_o_naianei = &self.mekameka[i];
            let poraka_o_mua = &self.mekameka[i - 1];
            if poraka_o_naianei.hash != Poraka::tatauria_hash(
                poraka_o_naianei.taupanga,
                poraka_o_naianei.wa_timestamp,
                &poraka_o_naianei.hash_o_mua,
                &poraka_o_naianei.raraunga,
                poraka_o_naianei.nonce,
            ) || poraka_o_naianei.hash_o_mua != poraka_o_mua.hash {
                return false;
            }
        }
        true
    }
}

fn wa_o_naianei() -> u128 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let wa_o_naianei = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    wa_o_naianei.as_secs() as u128 * 1000 + wa_o_naianei.subsec_millis() as u128
}
