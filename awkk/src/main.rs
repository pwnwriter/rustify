use std::process::Command;

fn main() {
    let output = Command::new("awk")
        .arg(r#"{print $1, $3}"#)
        .arg("myfile.txt")
        .output()
        .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
