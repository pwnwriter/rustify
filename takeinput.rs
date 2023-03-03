// Takes user input of first argument and prints back.
//

use std::env;

fn takeinput() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Print first command line argument
        if args.len() < 2 {
            println!("Please provide a command line argument");
        }
            else {
        println!("You entered: {}", args[1]);   
    }

}

fn main(){
        takeinput();

}
