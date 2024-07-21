// src/main.rs

mod compile;
mod consensus;
mod interpretation;
mod manage;

use crate::consensus::{Purotu, RopuRaraunga, whakamana, tumomo_hoko};
use crate::interpretation::{TauiraHanganga, Whakamaramatanga, whakamaramatia_korero};
use crate::manage::HangangaKonae;

fn main() {
    let mut ropu_raraunga = RopuRaraunga {
        // Whakatakotoria ngā āpure i konei
    };
    let taura = Purotu {
        // Whakatakotoria ngā āpure i konei
    };

    whakamana(&mut ropu_raraunga, taura);

    let tauira = TauiraHanganga {
        apure1: "Tauira".to_string(),
        apure2: 42,
    };

    let whakamaramatanga = Whakamāramatanga {
        apure1: "Whakamāramatanga".to_string(),
        apure2: 24,
    };

    // Test interpretation
    whakamaramatia_korero("rerehangu");

    // Manage file example
    let konae = HangangaKonae::hou("Tauira Konae".to_string(), 1024);
    konae.tapirihia_kōnae("tauira.txt");
    konae.muku_kōnae("tauira.txt");
    konae.rarangi_konae();
        }
