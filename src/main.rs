use std::collections::HashMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

struct HttpRequest {
    method: String,
    path: String,
    version: String,
    headers: HashMap<String, String>,
}

fn parse_http_request(request_str: &str) -> Option<HttpRequest> {
    let mut lines = request_str.lines();

    let first_line = lines.next()?;
    let parts: Vec<&str> = first_line.split_whitespace().collect();

    if parts.len() != 3 {
        return None;
    }

    let method = parts[0].to_string();
    let path = parts[1].to_string();
    let version = parts[2].to_string();

    let mut headers = HashMap::new();

    for line in lines {
        if line.is_empty() {
            break;
        }

        if let Some(colon_pos) = line.find(':') {
            let key = line[..colon_pos].trim().to_string();
            let value = line[colon_pos + 1..].trim().to_string();
            headers.insert(key, value);
        }
    }

    Some(HttpRequest {
        method,
        path,
        version,
        headers,
    })
}

async fn handle_http(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer).await {
        Ok(bytes_read) => {
            let request_str = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("Raw request:\n{}", request_str);

            let request = match parse_http_request(&request_str) {
                Some(req) => req,
                None => {
                    eprintln!("Failed to parse request");
                    return;
                }
            };

            println!("Method: {}", request.method);
            println!("Path: {}", request.path);
            println!("Version: {}", request.version);
            println!("Headers:");
            for (key, value) in &request.headers {
                println!("  {}: {}", key, value);
            }

            let (status, body) = match request.path.as_str() {
                "/" => ("200 OK", "hello world"),
                "/health" => ("200 OK", "ok"),
                _ => ("404 NOT FOUND", "404 - Not Found"),
            };

            let response = format!(
                "HTTP/1.1 {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                status,
                body.len(),
                body
            );

            println!("Sending response: {}\n", status);
            stream.write_all(response.as_bytes()).await.unwrap();
            stream.flush().await.unwrap();
        }
        Err(e) => {
            eprintln!("Failed to read from connection: {}", e);
        }
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    println!("HTTP server listening on http://127.0.0.1:8000");

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                println!("New connection from: {}", addr);
                handle_http(stream).await;
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
