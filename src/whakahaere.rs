// src/whakahaere.rs

pub struct HangangaKonae {
    // Whakatakotoria nga apure i konei
    pub ingoa: String,
    pub rahi: u64,
}

impl HangangaKonae {
    pub fn hou(ingoa: String, rahi: u64) -> Self {
        HangangaKonae { ingoa, rahi }
    }

    pub fn tapirihia_konae(&self, konae: &str) {
        // Rautaki tapiri konae i konei
        println!("Konae '{}' kua tapirihia ki '{}'", konae, self. ingoa);
    }

    pub fn muku_konae(&self, konae: &str) {
        // Rautaki muku konae i konei
        println!("Konae '{}' kua mukua i '{}'", konae, self. ingoa);
    }

    pub fn rarangi_konae(&self) {
        // Rautaki rarangi konae i konei
        println!("Rarangi konae kei roto i '{}'", self. ingoa);
    }
}

pub fn whakahaere_konae() {
    // Tinana o te mahi i konei
    let konae = HangangaKonae::hou("Tauira Konae".to_string(), 1024);
    konae.tapirihia_konae("tauira.txt");
    konae.muku_konae("tauira.txt");
    konae.rarangi_konae();
}
