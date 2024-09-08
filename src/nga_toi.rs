use std::process::Command;
use std::io::{self, Write};

pub fn hanga_art_nga_terminal(text: &str) -> io::Result<String> {
    let output = Command::new("python3")
        .arg("art_generator.py")
        .arg(text)
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "I hinga te hōtaka Python"));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

