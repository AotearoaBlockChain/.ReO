mod compile;
mod consensus;
mod interpretation;
mod manage;
mod reo;
mod crypto;
mod blockchain;
mod network;

use crate::consensus::{Purotu, RopuRaraunga, whakamana, tumomo_hoko};
use crate::interpretation::{TauiraHanganga, Whakamaamatanga, whakamaramatia_korero};
use crate::manage::HangangaKonae;
use crate::crypto::{hangaia_kiwaha_matua, waitohua_raraunga, whakau_waitohu};
use crate::reo::{ReoScript, whakamuna_raraunga, hangaia_hmac, tapirihia_konae, mukua_konae, rarangi_konae};
use crate::blockchain::{Poraka, WhatungaPoraka};
use crate::network::WhatungaKnode;

fn main() {
    let commands = vec![
        ("whakamuna_raraunga", vec!["etahi raraunga hei whakamuna".to_string()]),
        ("hangaia_hmac", vec!["ki_muna".to_string(), "etahi raraunga hei waitohu".to_string()]),
        ("tapirihia_konae", vec!["tauira.txt".to_string()]),
        ("mukua_konae", vec!["tauira.txt".to_string()]),
        ("rarangi_konae", vec![]),
    ];

    for (waehere, params) in commands {
        let hotaka = ReoScript::hou(waehere, params);
        hotaka.whakahaere();
    }

    // Example ECDSA usage
    match hangaia_kiwaha_matua() {
        Ok((ki_muna, ki_tumatanui)) => {
            let raraunga = b"tauira raraunga".to_vec();
            match waitohua_raraunga(&ki_muna, &raraunga) {
                Ok(waitohu) => {
                    match whakau_waitohu(&ki_tumatanui, &raraunga, &waitohu) {
                        Ok(he_tika) => println!("He tika te waitohu: {}", he_tika),
                        Err(e) => println!("Hapa i te whakau waitohu: {}", e),
                    }
                },
                Err(e) => println!("Hapa i te waitohua raraunga: {}", e),
            }
        },
        Err(e) => println!("Hapa i te waihanga kiwaha matua: {}", e),
    }

    // Blockchain example
    let mut whatunga_poraka = WhatungaPoraka::hou();
    whatunga_poraka.tapiri_poraka("Raraunga tuarua".to_string());
    if whatunga_poraka.he_tika_te_mekameka() {
        println!("He tika te mekameka poraka");
    } else {
        println!("Kua kino te mekameka poraka");
    }

    // Network example
    let knode = WhatungaKnode::hou("127.0.0.1:8080".to_string());
    knode.timata();
}
