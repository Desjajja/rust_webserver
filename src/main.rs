use std::io::{Write, Read};
use std::net::{TcpListener, TcpStream};
use http_server_starter_rust::response::{get_response, Status, ContentType};
use http_server_starter_rust::request::{HttpRequest, parse_request};
use threadpool::ThreadPool;
use std::path::PathBuf;

fn main() {

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
            let mut buffer = [0u8; 2048] ; // 2KB sized buffer for header
            stream.read(& mut buffer).unwrap();
            let request = String::from_utf8_lossy(& buffer);

            if let Some(parsed_req) = parse_request(&request) {
                let method = parsed_req.method.unwrap().to_lowercase();
                let uri = parsed_req.path.unwrap();
                let response = match method {
                    method if method == "post" => {
                        match uri {
                            uri if uri.starts_with("/files") => post_file(parsed_req, root_dir),
                            _ => handle_404(),
                        }
                    },
                    method if method =="get" => {
                        match uri {
                            uri if uri == "/" => handle_ok(),
                            uri if uri.starts_with("/echo") => handle_echo(parsed_req),
                            uri if uri.starts_with("/user-agent") => handle_user_agent(parsed_req),
                            uri if uri.starts_with("/files") => get_files(parsed_req, root_dir),
                            _ => handle_404(),
                        }
                    },
                    _ => handle_404(),
                };
                stream.write_all(response.as_bytes()).unwrap();  
            }
                      }

fn post_file(req: HttpRequest, root_dir: Option<String>) -> String {
    let filename = req.path.unwrap().split('/').last().unwrap().to_owned();
    let mut path_buffer = PathBuf::new();
    path_buffer.push(PathBuf::from(root_dir.unwrap_or_default()));
    path_buffer.push(PathBuf::from(filename));
    match std::fs::File::create(path_buffer) {
        Ok(mut f) => f.write_all(req.body.unwrap().as_bytes()).unwrap(),
        Err(_) => return handle_404(),
    }
    handle_201()
}

fn handle_201() -> String {
    get_response(Status::Created, ContentType::None, None)
}

fn handle_echo(req: HttpRequest) -> String {
    let content = req.path.unwrap().split_once("/echo/").unwrap().1.to_owned();
    get_response(Status::OK, ContentType::TextPlain, Some(content))
}

fn handle_ok() -> String {
    get_response(Status::OK, ContentType::Unknown, None)
}

fn handle_user_agent(req: HttpRequest) -> String{
    let content = req.header_fields.get("user-agent").unwrap().to_owned().to_owned();
    get_response(Status::OK, ContentType::TextPlain, Some(content))
}

fn get_files(req: HttpRequest, root_dir: Option<String>) -> String {
    let filename = req.path.unwrap().split('/').last().unwrap().to_owned();
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