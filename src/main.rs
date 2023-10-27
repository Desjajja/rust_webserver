// Uncomment this block to pass the first stage
use std::net::TcpListener;
use std::io::{BufReader, BufRead, Write};
use regex::Regex;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let buf_reader = BufReader::new(&stream);
                let header = buf_reader.lines().next().unwrap().unwrap();
                let re = Regex::new(r"\/\w+\s").unwrap();
                let response = match re.captures(&header) {
                    Some(caps) => {
                        let status_line = "HTTP/1.1 200 OK\r\n\r\n";
                        let content_type = "Content-Type: text/plain\r\n\r\n";
                        let content = &caps[0][1..].trim();
                        let length = content.len();
                        format!("{status_line}{content_type}{length}{content}\r\n\r\n")
                    }
                    _ => String::from("HTTP/1.1 404 Not Found\r\n\r\n")
                };
                
                stream.write_all(response.as_bytes()).unwrap();
                println!("responded to connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
