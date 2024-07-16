mod interpretation;
mod compile;
mod manage;

fn main() {
    let command = "rerehāngū"; // Example command, you can replace this with actual input
    interpretation::interpret(command);

    let script = "example script"; // Example script, you can replace this with actual script
    compile::compile(script);
    manage::run(script);
    manage::debug(script);
    manage::test(script);
}
