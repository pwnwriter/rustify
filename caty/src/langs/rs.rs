use synoptic::{Token, Highlighter};
use termion::color;

pub fn highlight(to_hl: &str) {
    // Build the rust syntax highlighter
    let mut highlight = Highlighter::new();

    // Keywords
    highlight.join(&["fn", "return", "pub"], "keyword").unwrap();
    highlight.join(&["bool"], "type").unwrap();
    highlight.join(&["true", "false"], "boolean").unwrap();

    highlight.join(&["if", "else if", "else"], "statement").unwrap();

    // Comments
    highlight.add(r"(?m)(//.*)$", "comment").unwrap();
    highlight.add(r"(?ms)/\*.*?\*/", "comment").unwrap();

    // Strings
    highlight.add("\".*?\"", "string").unwrap();

    // Identifiers
    highlight.add(r"([a-z_][A-Za-z0-9_]*)\s*\(", "identifier").unwrap();

    // Macro definitions
    highlight.add(r"([a-z_][A-Za-z0-9_]*!)\s*", "macro").unwrap();

    // Run highlighter
    let highlighting = highlight.run(&to_hl);

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
                    "comment" => print!("{}", color::Fg(color::Black)),
                    "string" => print!("{}", color::Fg(color::Green)),
                    "keyword" => print!("{}", color::Fg(color::Blue)),
                    "type" => print!("{}", color::Fg(color::LightMagenta)),
                    "boolean" => print!("{}", color::Fg(color::LightGreen)),
                    "identifier" => print!("{}", color::Fg(color::Yellow)),
                    "statement" => print!("{}", color::Fg(color::Red)),
                    "macro" => print!("{}", color::Fg(color::Magenta)),
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

}
