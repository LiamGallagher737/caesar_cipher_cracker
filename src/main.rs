use std::io;
use caesar_cipher_cracker::caesar_shift;

fn main() {
    // Get the encrypted message from the user.
    println!("Enter the encrypted message:");
    let mut message = String::new();
    io::stdin().read_line(&mut message).unwrap();

    // Try all possible Caesar cipher shifts and print the decrypted messages.
    for i in 1..26 {
        println!("{}: {}", i, caesar_shift(&message, i));
    }
}
