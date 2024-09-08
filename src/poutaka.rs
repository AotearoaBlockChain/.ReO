mod nga_toi; // Import the module using the Māori name

fn main() {
    // Call the function from nga_toi
    let art = nga_toi::hanga_art_nga_terminal("Kia ora, Ao!").unwrap();
    println!("{}", art);
}

