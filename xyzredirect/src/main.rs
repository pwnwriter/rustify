use std::env;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};
use std::time::Duration;

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 512];
    stream.read(&mut buf).unwrap();

    let response = "HTTP/1.1 301 Moved Permanently\r\nLocation: https://pwnwriter.xyz\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if the user provided a port number as a command line argument
    if args.len() != 2 {
        println!("Usage: ./server <port>");
        return;
    }

    let port = &args[1];
    let message = format!("Status : Legend, Server listening on port {}", port);

    loop {
        print!("\r{}", message);

        // Animate the message by adding dots to the end
        let mut dots = 0;
        loop {
            for _ in 0..dots {
                print!(".");
            }
            std::io::stdout().flush().unwrap();

            dots = (dots + 1) % 4;
            thread::sleep(Duration::from_millis(500));
        }

        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            thread::spawn(|| {
                handle_client(stream);
            });
        }
    }
}

