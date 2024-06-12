use std::fs;
use std::io::{Read, Result, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() -> Result<()> {
    const HOST: &str = "127.0.0.1";
    const PORT: &str = "8080";
    let address = format!("{HOST}:{PORT}");

    let listener = TcpListener::bind(address)?;

    println!("Server running on http://{}", listener.local_addr()?);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move || {
             handle_connection(stream);
        });
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap();
    let request_method = request_line.split_whitespace().nth(0).unwrap();
    let path = request_line.split_whitespace().nth(1).unwrap();

    println!("Request: {}", request);

    match request_method {
        "GET" => handle_get_method(path, &mut stream),
        "POST" => handle_post_method(&request, &mut stream, &buffer),
        _ => handle_not_found(&mut stream),
    }
}

fn handle_get_method(path: &str, stream: &mut TcpStream) {
    let (status_line, filename) = match path {
        "/" => ("HTTP/1.1 200 OK", "index.html"),
        "/about" => ("HTTP/1.1 200 OK", "about.html"),
        "/app" => ("HTTP/1.1 200 OK", "app.js"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let content_type = get_content_type(filename);
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "HTTP/1.1 {} OK\r\nContent-Type: {}; charset=UTF-8\r\n\r\n{}",
        status_line, content_type, contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
fn handle_post_method(request: &str, stream: &mut TcpStream, buffer: &[u8]) {
    let post_body = request.split("\r\n\r\n").nth(1).unwrap_or("");
    println!("POST request to {}", post_body);
    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=UTF-8\r\n\r\nPOST received! {}", post_body);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
fn handle_not_found(stream: &mut TcpStream) {
    let response = "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/plain; charset=UTF-8\r\n\r\n404 Not Found";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
fn get_content_type(filename: &str) -> &str {
    if filename.ends_with(".html") {
        "text/html"
    } else if filename.ends_with(".css") {
        "text/css"
    } else if filename.ends_with(".js") {
        "application/javascript"
    } else if filename.ends_with(".jpg") {
        "image/jpeg"
    } else if filename.ends_with("png") {
        "image/png"
    } else {
        "text/plain"
    }
}
