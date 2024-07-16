use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

// Taupānga i te Tauira Pūrotu
#[derive(Debug, Clone)]
struct Pūrotu {
    tohu: u64,
    wātaka: u64,
    raraunga: Vec<Hoko>,
    tawhito_whakamutunga: String,
    nonce: u64,
}

// Tauira Hoko
#[derive(Debug, Clone)]
struct Hoko {
    kaituku: String,
    kaiwhiwhi: String,
    moni: u64,
}

// Rōpū Whakamana mō te Rite Whakamāori
#[derive(Debug, Clone)]
struct Whakamana {
    wāhitau: String,
    moni: u64,
}

// Pūrotu Pūraurau
#[derive(Debug)]
struct PūrotuPūraurau {
    taura: Vec<Pūrotu>,
    hoko_ā-nāianei: Vec<Hoko>,
    whakamana: Vec<Whakamana>,
    uaua: usize,
}

impl PūrotuPūraurau {
    // Kaitautoko
    fn hōmai(uaua: usize) -> Self {
        let mut pūrotu_pūraurau = PūrotuPūraurau {
            taura: Vec::new(),
            hoko_ā-nāianei: Vec::new(),
            whakamana: Vec::new(),
            uaua,
        };
        pūrotu_pūraurau.tāpiri_tūmomo_whakamutanga_mōtehoko();
        pūrotu_pūraurau
    }

    // Tāpiri i te pūtahi tūmomo
    fn tāpiri_pūtahi(&mut self, hoko: Hoko) {
        self.hoko_ā-nāianei.push(hoko);
    }

    // Tāpiri i te pūtahi tūmomo whakamutanga mō te hoko
    fn tāpiri_tūmomo_whakamutanga_mōtehoko(&mut self) {
        let hoko_tūmomo = Hoko {
            kaituku: String::from("Kaituku"),
            kaiwhiwhi: String::from("Kaiwhiwhi"),
            moni: 0,
        };
        self.tāpiri_pūtahi(hoko_tūmomo);
    }
}

// Kaitūmuaki mō te Rōpū Raraunga
#[derive(Debug, Clone)]
struct RōpūRaraunga {
    taura: Vec<Pūrotu>,
    hoko_ā-nāianei: Vec<Hoko>,
    whakamana: Vec<Whakamana>,
    uaua: usize,
}

impl RōpūRaraunga {
    // Kaitautoko
    fn hōmai(uaua: usize) -> Self {
        let mut rōpū_raraunga = RōpūRaraunga {
            taura: Vec::new(),
            hoko_ā-nāianei: Vec::new(),
            whakamana: Vec::new(),
            uaua,
        };
        rōpū_raraunga.tāpiri_tūmomo_whakamutanga_mōtehoko();
        rōpū_raraunga
    }

    // Tāpiri i te pūtahi tūmomo
    fn tāpiri_pūtahi(&mut self, hoko: Hoko) {
        self.hoko_ā-nāianei.push(hoko);
    }

    // Tāpiri i te pūtahi tūmomo whakamutanga mō te hoko
    fn tāpiri_tūmomo_whakamutanga_mōtehoko(&mut self) {
        let hoko_tūmomo = Hoko {
            kaituku: String::from("Kaituku"),
            kaiwhiwhi: String::from("Kaiwhiwhi"),
            moni: 0,
        };
        self.tāpiri_pūtahi(hoko_tūmomo);
    }
}

// Taupānga Pūraurau
#[derive(Debug)]
struct Pūraurau {
    taura: Vec<Pūrotu>,
    hoko_ā-nāianei: Vec<Hoko>,
    whakamana: Vec<Whakamana>,
    uaua: usize,
}

impl Pūraurau {
    // Kaitautoko
    fn hōmai(uaua: usize) -> Self {
        let mut pūraurau = Pūraurau {
            taura: Vec::new(),
            hoko_ā-nāianei: Vec::new(),
            whakamana: Vec::new(),
            uaua,
        };
        pūraurau.tāpiri_tūmomo_whakamutanga_mōtehoko();
        pūraurau
    }

    // Tāpiri i te pūtahi tūmomo
    fn tāpiri_pūtahi(&mut self, hoko: Hoko) {
        self.hoko_ā-nāianei.push(hoko);
    }

    // Tāpiri i te pūtahi tūmomo whakamutanga mō te hoko
    fn tāpiri_tūmomo_whakamutanga_mōtehoko(&mut self) {
        let hoko_tūmomo = Hoko {
            kaituku: String::from("Kaituku"),
            kaiwhiwhi: String::from("Kaiwhiwhi"),
            moni: 0,
        };
        self.tāpiri_pūtahi(hoko_tūmomo);
    }
}

fn whakamana(&mut rōpūraraunga, wāhanga: Pūrotu) {
    rōpūraraunga.taura.push(wāhanga);
}

fn tūmomo_hoko() -> Hoko {
    Hoko {
        kaituku: String::from("Kaituku"),
        kaiwhiwhi: String::from("Kaiwhiwhi"),
        moni: 0,
    }
}

fn whakamana_rahinga() -> Whakamana {
    Whakamana {
        wāhitau: String::from("Wāhitau"),
        moni: 0,
    }
}

fn main() {
    // Timata pūraurau me te taumaha uaua
    let mut pūraurau = Pūraurau::hōmai(4);

    // Tāpiri kaitautoko mō te Rite Whakamāori
    whakamana(&mut pūraurau, Pūrotu::tāpiri_tūmomo_whakamutanga_mōtehoko("Rite Whakamāori 1"), 100);
    whakamana(&mut pūraurau, Pūrotu::tāpiri_tūmomo_whakamutanga_mōtehoko("Rite Whakamāori 2"), 200);

    // Tāpiri tūmomomomo ki ngā tūmomo o nāianei
    let tūmomo1 = tūmomo_hoko();
    pūraurau.tāpiri_pūtahi(tūmomo1.clone());

    let tūmomo2 = tūmomo_hoko();
    pūraurau.tāpiri_pūtahi(tūmomo2.clone());

    // Kī kia whakaatuhia he pūrotu hōu mā te Rite Whakamāori
    let raro = pūraurau.taura.hoatohanga(0).unwrap();
    let rōpūraraunga_rua = Pūrotu::tūmomo_hoko(raro);

    // Mōhiohio whakamutunga rōpū me ngā kaitautoko
    println!("Rōpū Raraunga: {:#?}", pūraurau.taura);
    println!("Kaitautoko: {:#?}", pūraurau.whakamana);
}
