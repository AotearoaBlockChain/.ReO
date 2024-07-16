mod interpretation;
mod compile;
mod manage;
mod consensus; // Include your consensus.rs file

fn main() {
    let command = "rerehangu"; // Example command, you can replace this with actual input
    interpretation::interpret(command);

    let script = "example script"; // Example script, you can replace this with actual script
    compile::compile(script);
    manage::run(script);
    manage::debug(script);
    manage::test(script);

    // Example usage of blockchain logic from consensus.rs
    // Initialize blockchain with difficulty level
    let mut blockchain = consensus::Blockchain::new(4);

    // Add validators for Proof of Stake
    blockchain.add_validator(String::from("Validator1"), 100);
    blockchain.add_validator(String::from("Validator2"), 200);

    // Add transactions to current transactions
    let transaction1 = consensus::Transaction {
        sender: String::from("Alice"),
        receiver: String::from("Bob"),
        amount: 50,
    };
    blockchain.add_transaction(transaction1.clone());

    let transaction2 = consensus::Transaction {
        sender: String::from("Bob"),
        receiver: String::from("Alice"),
        amount: 30,
    };
    blockchain.add_transaction(transaction2.clone());

    // Mine a new block using Proof of Work
    let last_block = blockchain.chain.last().unwrap();
    let new_block_pow = blockchain.proof_of_work(last_block);
    blockchain.add_block(new_block_pow);

    // Output blockchain and validators
    println!("Blockchain: {:#?}", blockchain.chain);
    println!("Validators: {:#?}", blockchain.validators);
}
