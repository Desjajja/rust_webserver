// Uncomment this block to pass the first stage
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

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
                let uri = header.split(' ').nth(1).unwrap();
                let response:String = match uri {
                    "/" => "HTTP/1.1 200 OK\r\n\r\n".to_owned(),
                    _ => {
                        if uri.contains("/echo/") {
                            let content = uri.split_once("/echo/").unwrap().1;
                            format!(r"HTTP/1.1 200 OK\r\n\r\nContent-Type: text/plain\r\n\r\nContent-Length: {}\r\n\r\n{}\r\n\r\n",content.len(), content)
                        } else {
                            "HTTP/1.1 404 Not Found\r\n\r\n".to_owned()
                        }
                    }
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
