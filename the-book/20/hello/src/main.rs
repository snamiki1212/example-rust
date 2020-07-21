use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, contents) = if buffer.starts_with(get) {
        let contents = fs::read_to_string("index.html").expect("cannot open file");
        let status_line = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",
            contents.len()
        );
        (status_line, contents)
    } else {
        let contents = fs::read_to_string("404.html").expect("cannot open file");
        let status_line = format!(
            "HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n",
            contents.len()
        );
        (status_line, contents)
    };

    let response = format!(
        "{}{}",
        status_line,
        contents
    );
    stream
        .write(response.as_bytes())
        .expect("faile to write into stream data");
    stream.flush().unwrap();
}
