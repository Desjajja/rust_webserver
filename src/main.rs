// Uncomment this block to pass the first stage
use std::net::TcpListener;
use std::io::{BufReader, BufRead, Write};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let buf_reader = BufReader::new(&stream);
                let _header = buf_reader.lines().next().unwrap().unwrap();

                let status_line = b"HTTP/1.1 200 OK\r\n\r\n";
                stream.write_all(status_line).unwrap();
                println!("responded to connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}