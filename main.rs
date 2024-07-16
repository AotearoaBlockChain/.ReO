mod interpretation;
mod compile;
mod manage;

use std::fs;

fn main() {
    // Read the test script from the file
    let script = fs::read_to_string("test.reo")
        .expect("Unable to read file");

    // Split the script into lines
    let lines: Vec<&str> = script.lines().collect();

    // Interpret each line
    for line in lines {
        interpretation::interpret(line);
    }

    // Compile the script
    compile::compile(&script);

    // Run the script
    manage::run(&script);

    // Debug the script
    manage::debug(&script);

    // Test the script
    manage::test(&script);
}