// src/main.rs

mod compile;
mod consensus;
mod interpretation;
mod manage;
mod reo;

use crate::consensus::{Purotu, RopuRaraunga, whakamana, tumomo_hoko};
use crate::interpretation::{TauiraHanganga, Whakamaamatanga, whakamaramatia_korero};
use crate::manage::HangangaKonae;

fn main() {
    let code = "whakamuna_raraunga";
    let script =
reo::ReoScript::new(code);
    script.execute();

    let code = "hangaia_hmac";
    let script =
reo::ReoScript::new(code);
    script.execute();

    let code = "analyze_data";
    let script = 
reo::ReoScript::new(code);
    script.execute();

    let mut ropu_raraunga = RopuRaraunga {
        // Initialize fields here if needed
    };
    let taura = Purotu {
        // Initialize fields here if needed
    };

    whakamana(&mut ropu_raraunga, taura);

    let tauira = TauiraHanganga {
        apure1: "Tauira".to_string(),
        apure2: 42,
    };

    let whakamaamatanga = Whakamaamatanga {
        apure1: "Whakamaamatanga".to_string(),
        apure2: 24,
    };

    // Test interpretation
    whakamaramatia_korero("rerehangu");

    // Manage file example
    let konae = HangangaKonae::hou("Tauira Konae".to_string(), 1024);
    konae.tapirihia_konae("tauira.txt");
    konae.muku_konae("tauira.txt");
    konae.rarangi_konae();
}
