use std::env;
use std::env::args;

fn main() {

    // Collect arguments
    let arguments: Vec<_> = env::args().collect();

    // Create symbol variable
    let symbol;

    // Check if user given an argument
    // If true, the argument will be used as the symbol,
    if arguments.len() > 1 {
        let symbol_arg:String = args().nth(1).unwrap();
        symbol = symbol_arg;

    // Otherwise check for an environment variable 'CCC_SYMBOL',
    // If the variable exists, use it as the symbol, if not,
    // a rust symbol will be used.
    } else {
        match env::var("CCC_SYMBOL") {
            Ok(val) => symbol = val,
            Err(_e) => symbol = "".to_string(),
        }
    }

    // The rest is iterating the 'count variable and using
    // Escape characters to colorize the symbol

    let mut count = 0u32;

    print!("\n");
    loop {
        count += 1;
        print!(" \x1B[{}m{} \x1B[m", 29 + count, symbol);
        if count == 8 {
            break;
        }
    }
    print!("\n");

    count = 0;

    loop {
        count += 1;
        print!(" \x1B[{}m{} \x1B[m", 89 + count, symbol);
        if count == 8 {
            break;
        }
    }

    print!("\n\n");
}
