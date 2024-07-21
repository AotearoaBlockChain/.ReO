// src/manage.rs

pub struct HangangaKōnae {
    // Whakatakotoria ngā āpure i konei
    pub ingoa: String,
    pub rahi: u64,
}

impl HangangaKōnae {
    pub fn hou(ingoa: String, rahi: u64) -> Self {
        HangangaKōnae { ingoa, rahi }
    }

    pub fn tāpirihia_kōnae(&self, kōnae: &str) {
        // Rautaki tāpiri kōnae i konei
        println!("Kōnae '{}' kua tāpirihia ki '{}'", kōnae, self. ingoa);
    }

    pub fn muku_kōnae(&self, kōnae: &str) {
        // Rautaki muku kōnae i konei
        println!("Kōnae '{}' kua mukua i '{}'", kōnae, self. ingoa);
    }

    pub fn rārangi_kōnae(&self) {
        // Rautaki rārangi kōnae i konei
        println!("Rārangi kōnae kei roto i '{}'", self. ingoa);
    }
}

pub fn whakahaere_kōnae() {
    // Tinana o te mahi i konei
    let kōnae = HangangaKōnae::hou("Tauira Kōnae".to_string(), 1024);
    kōnae.tāpirihia_kōnae("tauira.txt");
    kōnae.muku_kōnae("tauira.txt");
    kōnae.rārangi_kōnae();
}
