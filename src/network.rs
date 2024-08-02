use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

pub struct WhatungaKnode {
    pub wahi_tautuhinga: String,
    pub hoa: Vec<String>,
}

impl WhatungaKnode {
    pub fn hou(wahi_tautuhinga: String) -> Self {
        WhatungaKnode {
            wahi_tautuhinga,
            hoa: vec![],
        }
    }

    pub fn timata(&self) {
        let whakarongo = TcpListener::bind(&self.wahi_tautuhinga).unwrap();
        for roma in whakarongo.incoming() {
            let roma = roma.unwrap();
            self.whakahaere_hononga(roma);
        }
    }

    fn whakahaere_hononga(&self, mut roma: TcpStream) {
        let mut putunga = [0; 1024];
        roma.read(&mut putunga).unwrap();
        // Process incoming message
    }

    pub fn whakapaoho_karere(&self, karere: &str) {
        for hoa in &self.hoa {
            let mut roma = TcpStream::connect(hoa).unwrap();
            roma.write(karere.as_bytes()).unwrap();
        }
    }
      }
