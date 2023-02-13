use std::env::args;
use std::fs::File;
use std::io::Read;

use termion::color;

// Langs
mod langs;

fn main() {
    // Colorize text
    let fg_reset = color::Fg(color::Reset);
    let fg_red = color::Fg(color::Red);
    let fg_lred = color::Fg(color::LightRed);
    let fg_blue = color::Fg(color::Blue);

    // Collect command line arguments
    let args: Vec<String> = args().collect();

    // Exit if theres no arguments
    if args.len() > 1 {
    } else {
        print!("\nUsage: {}catty {}<file>{}\n\n", fg_red, fg_blue, fg_reset);
        return;
    }

    let filename = &args[1];
    match File::open(filename) {
        // The file is open
        Ok(mut file) => {

            let mut content = String::new();

            // Read file content to a string
            file.read_to_string(&mut content)
                .unwrap();

            // Detect file type by looking for the suffix
            if filename.ends_with(".rs") {
                langs::rs::highlight(&content);
            } else if filename.ends_with(".c") {
                langs::c::highlight(&content);
            } else {
                langs::default::highlight(&content);
            }

            // The file is automatically closed when it goes out of scope.
        },

        // Error handling.
        Err(err) => {
            print!("\nCan't open {}{}{} | {}\n\n",
                fg_lred, filename, fg_reset, err);
        },
    }
}
