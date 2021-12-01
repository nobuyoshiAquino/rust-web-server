use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        // A single stream represents an open connection 
        // between the client and the server.
        let stream = stream.unwrap();

        handle_connection(stream);

        // When stream goes out of scope and is dropped at the end
        // of the loop, the connection is closed as part of the drop
        // implementation.
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    // Read bytes from the TcpStream and put them in the buffer.
    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();

    // `flush` will wait and prevent the program
    // from continuing until all the bytes are written
    // to the connection.
    stream.flush().unwrap();
}