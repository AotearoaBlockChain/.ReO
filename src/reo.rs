use ring::digest::{Context, Digest, SHA256};
use ring::hmac;

// Whakamunatia ngā raraunga
pub fn whakamuna_raraunga(raraunga: &str) -> String {
    let mut context = Context::new(&SHA256);
    context.update(raraunga.as_bytes());
    let digest = context.finish();
    hex::encode(digest.as_ref())
}

// Waihangahia te HMAC
pub fn hangaia_hmac(ki: &str, raraunga: &str) -> String {
    let hmac_ki = hmac::Key::new(hmac::HMAC_SHA256, ki.as_bytes());
    let waitohu = hmac::sign(&hmac_ki, raraunga.as_bytes());
    hex::encode(waitohu.as_ref())
}

pub struct ReoScript {
    pub code: String,
}

impl ReoScript {
    pub fn new(code: &str) -> Self {
        ReoScript {
            code: code.to_string(),
        }
    }

    pub fn execute(&self) {
        if self.code.contains("tātari_raraunga") {
            tātari_raraunga();
        } else if self.code.contains("whakamuna_raraunga") {
            let raraunga = "ētahi raraunga hei whakamuna";
            let hash = whakamuna_raraunga(raraunga);
            println!("Kua whakamunatia ngā raraunga: {}", hash);
        } else if self.code.contains("hangaia_hmac") {
            let ki = "ki_muna";
            let raraunga = "ētahi raraunga hei waitohu";
            let hmac = hangaia_hmac(ki, raraunga);
            println!("Kua hangaia te HMAC: {}", hmac);
        } else {
            println!("Kāore he mahi mō tēnei kōwae.");
        }
    }
}

fn tātari_raraunga() {
    // Tauira mahi tātari raraunga
    let raraunga = vec![1, 2, 3, 4, 5];
    let tapeke: i32 = raraunga.iter().sum();
    println!("Kua tatauria ngā raraunga: {}", tapeke);

    let toharite = tapeke as f32 / raraunga.len() as f32;
    println!("Ko te toharite: {}", toharite);

    let tino_teitei = raraunga.iter().max().unwrap();
    println!("Ko te tino teitei: {}", tino_teitei);
}
