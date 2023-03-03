use std::env;
use base64::encode;

fn main() {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a string was provided as an argument
    if args.len() != 2 {
        println!("Usage: base64_encoder <string>");
        return;
    }

    // Encode the string to base64
    let base64_string = encode(&args[1]);

    // Print the encoded string
    println!("{}", base64_string);
}

