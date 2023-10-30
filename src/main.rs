// Uncomment this block to pass the first stage
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use http_server_starter_rust::response::{get_response, Status, ContentType};
use threadpool::ThreadPool;
use std::path::PathBuf;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let pool = ThreadPool::new(4);
    let mut args = std::env::args();
    let mut root_dir = None;
    if let Some(arg) = args.nth(1) {
        if arg == "--directory" {
            root_dir = args.last();
        }
    }
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let root_dir = root_dir.clone();
        pool.execute(|| {
            handle_connection(stream, root_dir);
        });
    }
}

fn handle_connection(mut stream: TcpStream, root_dir: Option<String> ) {

            let buf_reader = BufReader::new(&mut stream);
            let mut request = buf_reader.lines();
            let header = request.nth(0).unwrap().unwrap();
            let uri = header.split(' ').nth(1).unwrap();

            // let mut directory = None;

            let response = match uri {
                "/" => handle_ok(),
                uri if uri.starts_with("/echo") => handle_echo(uri),
                uri if uri.starts_with("/user-agent") => {
                    let user_agent = request.find(|line| {line
                        .as_ref()
                        .unwrap()
                        .to_lowercase()
                        .starts_with("user-agent")});
                    let user_agent = user_agent.unwrap().unwrap();
                    handle_user_agent(user_agent)
                },
                uri if uri.starts_with("/files") => handle_files(uri, root_dir),
                _ => handle_404(),
                };
                stream.write_all(response.as_bytes()).unwrap();
            }
            // println!("responded to connection");


fn handle_echo(uri: &str) -> String {
    let content = uri.split_once("/echo/").unwrap().1.to_owned();
    get_response(Status::OK, ContentType::TextPlain, Some(content))
}

fn handle_ok() -> String {
    get_response(Status::OK, ContentType::Unknown, None)
}

fn handle_user_agent(user_agent: String) -> String{
    let content = user_agent.split_once(' ').unwrap().1.to_owned();
    get_response(Status::OK, ContentType::TextPlain, Some(content))
}

fn handle_files(uri: &str, root_dir: Option<String>) -> String {
    let filename = uri.split('/').last().unwrap();
    let mut path_buffer = PathBuf::new();
    path_buffer.push(PathBuf::from(root_dir.unwrap_or_default()));
    path_buffer.push(PathBuf::from(filename));
    if !path_buffer.exists() {
        return handle_404();
    }   
    let content = std::fs::read_to_string(path_buffer).unwrap();
    get_response(Status::OK, ContentType::File, Some(content))
}

fn handle_404() -> String {
    get_response(Status::NotFound, ContentType::Unknown, None)
}