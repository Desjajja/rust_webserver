use std::ffi::IntoStringError;
// Uncomment this block to pass the first stage
use std::io::{BufRead, BufReader, Write, Read};
use std::net::{TcpListener, TcpStream};
use http_server_starter_rust::response::{get_response, Status, ContentType};
use threadpool::ThreadPool;
use std::path::{PathBuf, self};
use std::collections::HashMap;

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
#[derive(Debug)]
struct HttpRequest {
    method: Option<String>,
    path: Option<String>,
    version: Option<String>,
    header_fields: HashMap<String, String>,
    // content_length: usize,
    body: Option<String>,
}
fn parse_request<'a> (request: &'a str) -> Option<HttpRequest> {
    let mut method = None;
    let mut path = None;
    let mut version = None;
    let mut header_fields = HashMap::new();

    let request_split = request.split_once("\r\n\r\n")?;
    let (header, body) = request_split;

    for (idx, line) in header.lines().enumerate() {
        if idx == 0 {
            let mut metadata = line.split_whitespace();
            method = Some(metadata.next()?.to_owned());
            path = Some(metadata.next()?.to_owned());
            version = Some(metadata.next()?.to_owned());
        } else {
            let (k, v) = line.split_once(':')?;
            header_fields.insert(
                k.trim().to_lowercase().to_owned(), //keys are stored in lowercase
                v.trim().to_owned(),
            );
        }
    }
    let length = header_fields.get("content-length");
    let length:usize = match length {
        Some(length) => {
            length
                .parse()
                .unwrap()
        },
        _ => 0,
    };

        let body = match length{
            length if length > 0 => {
                Some(body[..length].to_owned())
            },
            _ => None,
        }; 
    Some(HttpRequest {header_fields, method, path, body, version})
}

fn handle_connection(mut stream: TcpStream, root_dir: Option<String> ) {
            let mut head_buffer = [0u8; 2048] ; // 2KB sized buffer for header
            stream.read(& mut head_buffer).unwrap();
            let request = String::from_utf8_lossy(& head_buffer);
            // if let Some(parsed_req) = parse_request(&request) {
            //     println!("{:?}", parsed_req);
            // }

            if let Some(parsed_req) = parse_request(&request) {
                let method = parsed_req.method.clone().unwrap().to_lowercase();
                let uri = parsed_req.path.clone().unwrap();
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
    handle_ok()
}

fn handle_echo(req: HttpRequest) -> String {
    let content = req.path.unwrap().split_once("/echo/").unwrap().1.to_owned();
    get_response(Status::OK, ContentType::TextPlain, Some(content))
}

fn handle_ok() -> String {
    get_response(Status::OK, ContentType::Unknown, None)
}

fn handle_user_agent(req: HttpRequest) -> String{
    let content = req.header_fields.get("user-agent").unwrap().to_owned();
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