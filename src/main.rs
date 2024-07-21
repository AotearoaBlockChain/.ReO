// src/main.rs

mod compile;
mod consensus;
mod interpretation;
mod manage;

use crate::consensus::{Puropu, RopuRaraunga, whakamana, tumomo_hoko};
use crate::interpretation::{TauiraHanganga, Whakamaamatanga, whakamaramatia_korero};
use crate::manage::HangangaKonae;

fn main() {
    let mut ropu_raraunga = RopuRaraunga {
        // Whakatakotoria nga apure i konei
    };
    let taura = Puropu {
        // Whakatakotoria nga apure i konei
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
