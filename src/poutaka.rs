mod nga_toi; // Import the module using the Māori name

fn main() {
    // Call the function from nga_toi
    let art = nga_toi::hanga_art_nga_terminal("Kia ora, Ao!").unwrap();
    println!("{}", art);
    }

    pub fn run_script(script: &str) {
    // Implement your script logic here
    println!("Running script:\n{}", script);
    }

// Function to display a message
pub fn panui_karere() {
    println!("Kia ora mai i te panui_karere!");
    }

// Function to add two numbers
pub fn tapiiri_tau(a: i32, b: i32) {
    println!("Ko te tapiiri i {} me {} ko {}", a, b, a + b);
    }

// Function to subtract two numbers
pub fn tango_tau(a: i32, b: i32) {
    println!("Ko te tango i {} mai i {} ko {}", a, b, a - b);
    }

// Function to multiply two numbers
pub fn whakanuia_tau(a: i32, b: i32) {
    println!("Ko te whakanuia i {} me {} ko {}", a, b, a * b);
    }

// Function to divide two numbers
pub fn wehe_tau(a: i32, b: i32) {
    if b == 0 {
        println!("He hapa: Kaore e taea te wehe i te tau-kore.");
    } else {
        println!("Ko te wehe i {} ma {} ko {}", a, b, a as f64 / b as f64);
    }

// Function to display help message
pub fn whakaatu_awhina() {
    println!("Nga whakahau e waatea ana:");
    println!("1. panui_karere");
    println!("2. tapiiri_tau <a> <b>");
    println!("3. tango_tau <a> <b>");
    println!("4. whakanuia_tau <a> <b>");
    println!("5. wehe_tau <a> <b>");
    println!("6. whakaatu_awhina");
    }
}

