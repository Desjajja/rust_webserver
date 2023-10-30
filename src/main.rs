// Uncomment this block to pass the first stage
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use http_server_starter_rust::response::{get_response, Status, ContentType};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let buf_reader = BufReader::new(&stream);
                let mut request = buf_reader.lines();
                let header = request.nth(0).unwrap().unwrap();
                let uri = header.split(' ').nth(1).unwrap();
                // let host: String = request.nth(2).unwrap().unwrap();
                let mut user_agent = request.filter(|line| {line.as_ref().unwrap().starts_with("User-Agent")});
                let (status, content_type, content) = match uri {
                    "/" => (Status::OK, ContentType::Unknown, None),
                    _ => {
                        if uri.starts_with("/echo") {
                            let content = uri.split_once("/echo/").unwrap().1.to_owned();
                            (Status::OK, ContentType::TextPlain, Some(content))
                        } else if uri.starts_with("/user-agent"){
                            let user_agent = user_agent.next().unwrap().unwrap();
                            let content = user_agent.split_once(' ').unwrap().1.to_owned();
                            println!("matched field:{}", user_agent.split_once(' ').unwrap().0);
                            (Status::OK, ContentType::TextPlain, Some(content))
                        }
                        else
                        {
                            (Status::NotFound, ContentType::Unknown, None)
                        }
                    }
                };
                let response = get_response(status, content_type, content);
                stream.write_all(response.as_bytes()).unwrap();
                println!("responded to connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}