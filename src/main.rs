use std::collections::HashMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

struct HttpRequest {
    method: String,
    path: String,
    version: u8,
    headers: HashMap<String, String>,
}

fn parse_http_request_httparse(buffer: &[u8]) -> Result<HttpRequest, String> {
    let mut headers_buf = [httparse::EMPTY_HEADER; 64];

    let mut req = httparse::Request::new(&mut headers_buf);

    match req.parse(buffer) {
        Ok(httparse::Status::Complete(_bytes_parsed)) => {
            let method = req.method
                .ok_or("Missing method")?
                .to_string();

            let path = req.path
                .ok_or("Missing path")?
                .to_string();

            let version = req.version
                .ok_or("Missing version")?;

            let mut headers = HashMap::new();
            for header in req.headers {
                let name = header.name.to_string();
                let value = String::from_utf8_lossy(header.value).to_string();
                headers.insert(name, value);
            }

            Ok(HttpRequest {
                method,
                path,
                version,
                headers,
            })
        }
        Ok(httparse::Status::Partial) => {
            Err("Incomplete request".to_string())
        }
        Err(e) => {
            Err(format!("Parse error: {}", e))
        }
    }
}

async fn handle_http(mut stream: TcpStream, addr: std::net::SocketAddr) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer).await {
        Ok(bytes_read) => {
            let request = match parse_http_request_httparse(&buffer[..bytes_read]) {
                Ok(req) => req,
                Err(e) => {
                    eprintln!("[{}] Failed to parse request: {}", addr, e);
                    return;
                }
            };

            println!("[{}] Method: {}", addr, request.method);
            println!("[{}] Path: {}", addr, request.path);
            println!("[{}] Version: HTTP/1.{}", addr, request.version);
            println!("[{}] Headers:", addr);
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

            println!("[{}] Sending response: {}\n", addr, status);
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
                tokio::spawn(async move {
                    handle_http(stream, addr).await;
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
