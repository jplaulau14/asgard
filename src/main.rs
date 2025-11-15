use std::collections::HashMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use httparse::{Request, Status, EMPTY_HEADER};

struct HttpRequest {
    method: String,
    path: String,
    version: u8,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

fn parse_http_request_httparse(buffer: &[u8]) -> Result<(HttpRequest, usize), String> {
    let mut headers_buf = [EMPTY_HEADER; 64];

    let mut req = Request::new(&mut headers_buf);

    match req.parse(buffer) {
        Ok(Status::Complete(bytes_parsed)) => {
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

            let request = HttpRequest {
                method,
                path,
                version,
                headers,
                body: Vec::new(),
            };

            Ok((request, bytes_parsed))
        }
        Ok(Status::Partial) => {
            Err("Incomplete request".to_string())
        }
        Err(e) => {
            Err(format!("Parse error: {}", e))
        }
    }
}

async fn read_request_body(
    stream: &mut TcpStream,
    headers: &HashMap<String, String>,
    initial_buffer: &[u8],
    headers_end: usize,
) -> Result<Vec<u8>, String> {
    let content_length = match headers.get("Content-Length")
        .or_else(|| headers.get("content-length"))
    {
        Some(len_str) => len_str
            .parse::<usize>()
            .map_err(|_| "Invalid Content-Length")?,
        None => {
            return Ok(Vec::new());
        }
    };

    if content_length == 0 {
        return Ok(Vec::new());
    }

    let body_in_buffer = &initial_buffer[headers_end..];
    let already_read = body_in_buffer.len();

    if already_read >= content_length {
        return Ok(body_in_buffer[..content_length].to_vec());
    }

    let mut body = body_in_buffer.to_vec();
    let remaining = content_length - already_read;

    let mut remaining_buffer = vec![0u8; remaining];
    stream
        .read_exact(&mut remaining_buffer)
        .await
        .map_err(|e| format!("Failed to read remaining body: {}", e))?;

    body.extend_from_slice(&remaining_buffer);

    Ok(body)
}

async fn handle_http(mut stream: TcpStream, addr: std::net::SocketAddr) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer).await {
        Ok(bytes_read) => {
            let (mut request, headers_end) = match parse_http_request_httparse(&buffer[..bytes_read]) {
                Ok((req, end)) => (req, end),
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

            request.body = match read_request_body(&mut stream, &request.headers, &buffer[..bytes_read], headers_end).await {
                Ok(body) => body,
                Err(e) => {
                    eprintln!("[{}] Failed to read body: {}", addr, e);
                    return;
                }
            };

            if !request.body.is_empty() {
                println!("[{}] Body ({} bytes): {}",
                    addr,
                    request.body.len(),
                    String::from_utf8_lossy(&request.body)
                );
            }

            let (status, response_body) = match request.path.as_str() {
                "/" => ("200 OK", "hello world".to_string()),
                "/health" => ("200 OK", "ok".to_string()),
                "/echo" => {
                    if request.body.is_empty() {
                        ("400 BAD REQUEST", "No body provided".to_string())
                    } else {
                        ("200 OK", String::from_utf8_lossy(&request.body).to_string())
                    }
                }
                _ => ("404 NOT FOUND", "404 - Not Found".to_string()),
            };

            let response = format!(
                "HTTP/1.1 {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                status,
                response_body.len(),
                response_body
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
