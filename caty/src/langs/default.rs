use synoptic::{Token, Highlighter};
use termion::color;

pub fn highlight(to_hl: &str) {
    let mut highlight = Highlighter::new();

    // Comments
    highlight.add(r"(?m)(#.*)$", "comment").unwrap();

    // Strings
    highlight.add("\".*?\"", "string").unwrap();

    // Identifiers
    highlight.add(r"([a-z_][A-Za-z0-9_]*)\s*\(", "identifier").unwrap();

    // Run highlighter
    let highlighting = highlight.run(&to_hl);

    print!("\n");

    // For each row
    for (c, row) in highlighting.iter().enumerate() {
        // Print line number (with padding)
        print!("{: >3} ▏ ", c);
        // For each token within each row
        for tok in row {
            // Handle the tokens
            match tok {
                // Handle the start token (start foreground colour)
                Token::Start(kind) => match *kind {
                    "comment" => print!("{}", color::Fg(color::LightWhite)),
                    "string" => print!("{}", color::Fg(color::Green)),
                    "identifier" => print!("{}", color::Fg(color::Yellow)),
                    _ => (),
                }
                // Handle a text token (print out the contents)
                Token::Text(txt) => print!("{}", txt),
                // Handle an end token (reset foreground colour)
                Token::End(_) => print!("{}", color::Fg(color::Reset)),
            }
        }
        // Prevent text being cut off without a newline
        println!("");
    }

    print!("\n");

}
