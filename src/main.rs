mod compile;
mod consensus;
mod interpretation;
mod manage;
mod reo;
mod crypto;

use crate::consensus::{Purotu, RopuRaraunga, whakamana};
use crate::interpretation::{TauiraHanganga, Whakamaamatanga, whakamaramatia_korero};
use crate::manage::HangangaKonae;
use crate::crypto::{hangaia_kiwaha_matua, waitohua_raraunga, whakau_waitohu};
use crate::reo::{ReoScript, whakamuna_raraunga, hangaia_hmac, tapirihia_konae, mukua_konae, rarangi_konae};

fn main() {
    let whakahau = vec![
        "whakamuna_raraunga",
        "hangaia_hmac",
        "tapirihia_konae",
        "mukua_konae",
        "rarangi_konae",
    ];

    for waehere in whakahau {
        let hotaka = ReoScript::hou(waehere);
        hotaka.whakahaere();
    }

    let waehere = "whakamuna_raraunga";
    let hotaka = ReoScript::hou(waehere);
    hotaka.whakahaere();

    let waehere = "hangaia_hmac";
    let hotaka = ReoScript::hou(waehere);
    hotaka.whakahaere();

    let waehere = "tātari_raraunga";
    let hotaka = ReoScript::hou(waehere);
    hotaka.whakahaere();

    let mut ropu_raraunga = RopuRaraunga {
        // Whakatūngia ngā āpure i konei mēnā e tika ana
    };
    let taura = Purotu {
        // Whakatūngia ngā āpure i konei mēnā e tika ana
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

    // Whakamātautau i te whakamārama
    whakamaramatia_korero("rerehangu");

    // Tauira whakahaere kōnae
    let konae = HangangaKonae::hou("Tauira Konae".to_string(), 1024);
    konae.tapirihia_konae("tauira.txt");
    konae.muku_konae("tauira.txt");
    konae.rarangi_konae();

    // Tauira ECDSA
    match hangaia_kiwaha_matua() {
        Ok((ki_muna, ki_tumatanui)) => {
            let raraunga = b"tauira raraunga";
            match waitohua_raraunga(&ki_muna, raraunga) {
                Ok(waitohu) => {
                    match whakaū_waitohu(&ki_tumatanui, raraunga, &waitohu) {
                        Ok(he_tika) => println!("He tika te waitohu: {}", he_tika),
                        Err(e) => println!("Hapa i te whakau waitohu: {}", e),
                    }
                },
                Err(e) => println!("Hapa i te waitohua raraunga: {}", e),
            }
        },
        Err(e) => println!("Hapa i te waihanga kiwaha matua: {}", e),
    }
}
