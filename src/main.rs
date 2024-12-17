use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::thread;

fn is_dark_mode() -> bool {
    let output = Command::new("defaults")
        .args(["read", "-g", "AppleInterfaceStyle"])
        .output()
        .unwrap_or_else(|_| panic!("failed to execute defaults command"));

    String::from_utf8_lossy(&output.stdout).trim() == "Dark"
}

fn handle_write(mut stream: TcpStream) {
    let status_line = "HTTP/1.1 200 OK";
    let contents = if is_dark_mode() { "dark" } else { "light" };
    eprintln!("Sending response: {}", contents);
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_client(stream: TcpStream) {
    handle_write(stream);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:12142").unwrap();
    println!("Listening for connections on port {}", 12142);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
