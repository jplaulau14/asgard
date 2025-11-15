use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_http(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let request = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("Request:\n{}", request);

            let body = "hello";
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                body.len(),
                body
            );

            println!("Sending response...");
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(e) => {
            eprintln!("Failed to read from connection: {}", e);
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("HTTP server listening on http://127.0.0.1:8000");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection from: {}", stream.peer_addr().unwrap());
                handle_http(stream);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
