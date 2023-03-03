use std::fs;
use std::time::Duration;

fn main() {
    // Read the uptime file from the proc filesystem
    let uptime_str = fs::read_to_string("/proc/uptime").expect("Error reading uptime file");

    // Parse the uptime from the file (the first number is the uptime in seconds)
    let uptime_seconds: f64 = uptime_str.split_whitespace().next().unwrap().parse().unwrap();

    // Convert the uptime to a Duration type
    let uptime = Duration::from_secs(uptime_seconds as u64);

    // Print the uptime in a human-readable format, with colors
    println!("\x1b[32mUptime:\x1b[0m {} days, {} hours, {} minutes, {} seconds",
             uptime.as_secs() / 86400,
             uptime.as_secs() / 3600 % 24,
             uptime.as_secs() / 60 % 60,
             uptime.as_secs() % 60);
}
