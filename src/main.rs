// Uncomment this block to pass the first stage
use regex::Regex;
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
                let re = Regex::new(r"\/\w*\s").unwrap();
                let response = match re.captures(&header) {
                    Some(caps) => {
                        let status_line = "HTTP/1.1 200 OK";
                        let content_type = "text/plain";
                        let content = &caps[0][1..].trim();
                        let length = content.len();
                        let response: String;
                        if length == 0 {
                            println!("length of content is 0");
                            response = format!(r"{status_line}\r\n\r\n}");
                        } else {
                            response = format!(
                                r"{status_line}\r\n\r\nContent-Type: {content_type}\r\n\r\nContent-Length: {length}\r\n\r\n{content}\r\n\r\n"
                            );
                        }
                        response
                    }
                    _ => String::from("HTTP/1.1 404 Not Found\r\n\r\n"),
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
