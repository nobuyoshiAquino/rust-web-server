use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        // A single stream represents an open connection 
        // between the client and the server.
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
        
        // When stream goes out of scope and is dropped at the end
        // of the loop, the connection is closed as part of the drop
        // implementation.
        println!("Shutting down.");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();

    // `flush` will wait and prevent the program
    // from continuing until all the bytes are written
    // to the connection.
    stream.flush().unwrap();
}
