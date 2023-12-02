use http_server_starter_rust::request::{parse_request, HttpRequest};
use http_server_starter_rust::response::*;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

const HOST_IP: &str = "127.0.0.1:4221";

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut args = std::env::args();
    let mut root_dir = None;
    if let Some(arg) = args.nth(1) {
        if arg == "--directory" {
            root_dir = args.last();
        }
    }

    let listener = TcpListener::bind(HOST_IP).await.unwrap();
    while let Ok((socket, _)) = listener.accept().await {
        let root_dir = root_dir.clone(); // TODO: change this to shared memo or something...
        tokio::spawn(async move {
            if let Err(err) = process_request(socket, root_dir).await {
                eprintln!("Error processing request: {:?}", err);
            }
        });
    }
    Ok(())
}

async fn process_request(
    mut socket: TcpStream,
    root_dir: Option<String>,
) -> Result<(), std::io::Error> {
    let mut reader = BufReader::new(&mut socket);
    let received = reader.fill_buf().await.unwrap();
    let request = String::from_utf8_lossy(received);

    let request = parse_request(&request).unwrap();
    let method = request.method.unwrap().to_lowercase();
    let uri = request.path.unwrap();
    let response = match method {
        method if method == "post" => match uri {
            uri if uri.starts_with("/files") => post_file(request, root_dir).await,
            _ => respond_405(),
        },
        method if method == "get" => match uri {
            uri if uri == "/" => respond_200(),
            uri if uri.starts_with("/echo") => handle_echo(request),
            uri if uri.starts_with("/user-agent") => handle_user_agent(request),
            uri if uri.starts_with("/files") => get_files(request, root_dir).await,
            uri if uri.starts_with("/photos") => list_file(root_dir),
            _ => respond_404(),
        },
        _ => respond_404(),
    };
    socket.write_all(&response).await?;
    Ok(())
}

#[derive(Serialize)]
struct ImageJson {
    id: usize,
    img_src: String,
}
// fn list_file(root_dir: Option<String>, content_type: ContentType) -> Vec<u8> // todo: show specific file types
fn list_file(root_dir: Option<String>) -> Vec<u8> {
    match root_dir {
        None => respond_500(),
        Some(root_dir) => {
            let mut path_buffer = PathBuf::new();
            path_buffer.push(PathBuf::from(root_dir));
            if let Ok(entries) = fs::read_dir(path_buffer) {
                let entries = entries
                    .enumerate()
                    .filter_map(|(id, entry)| match entry {
                        Ok(entry) => Some(ImageJson {
                            id: id,
                            img_src: HOST_IP.to_owned()
                                + "/files/"
                                + entry.file_name().to_str().unwrap(),
                        }),
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                let json = serde_json::to_string(&entries).unwrap();
                // respond_200_with_text(json)
                respond_200_with_content(json.into_bytes(), ContentType::TextPlain)
            } else {
                respond_500()
            }
        }
    }
}

async fn post_file(req: HttpRequest<'_>, root_dir: Option<String>) -> Vec<u8> {
    match root_dir {
        None => respond_500(),
        Some(root_dir) => {
            let filename = req.path.unwrap().split('/').last().unwrap().to_owned();
            let mut path_buffer = PathBuf::new();
            path_buffer.push(PathBuf::from(root_dir));
            path_buffer.push(PathBuf::from(filename));
            match File::create(path_buffer).await {
                Ok(mut f) => f.write_all(req.body.unwrap().as_bytes()).await.unwrap(),
                Err(_) => return respond_404(),
            }
            respond_201()
        }
    }
}

fn handle_echo(req: HttpRequest) -> Vec<u8> {
    let content = req.path.unwrap().split_once("/echo/").unwrap().1.to_owned();
    // respond_200_with_text(content)
    respond_200_with_content(content.into_bytes(), ContentType::TextPlain)
}

fn handle_user_agent(req: HttpRequest) -> Vec<u8> {
    let content = req
        .header_fields
        .get("user-agent")
        .unwrap()
        .to_owned()
        .to_owned();
    respond_200_with_content(content.into_bytes(), ContentType::TextPlain)
}

async fn get_files(req: HttpRequest<'_>, root_dir: Option<String>) -> Vec<u8> {
    match root_dir {
        None => respond_500(),
        Some(root_dir) => {
            let filename = req.path.unwrap().split('/').last().unwrap();
            let mut path_buffer = PathBuf::new();
            path_buffer.push(PathBuf::from(root_dir));
            path_buffer.push(PathBuf::from(filename));
            if !path_buffer.exists() {
                return respond_404();
            }

            let mut buffer = Vec::new();
            let mut f = File::open(path_buffer).await.unwrap();
            f.read_to_end(&mut buffer).await.unwrap();  // todo: optimize transfer with `chunked`, etc.
            let file_extension = filename.split_once('.').unwrap().1; // todo: check whether has extension name
            let content_type = match file_extension.to_lowercase().as_str() {
                // "png" => ContentType::Png, // todo: migrate this to `/display` or somewhat
                // "jpeg" | "jpg" => ContentType::Jpeg,
                _ => ContentType::Unknown,
            };
            respond_200_with_content(buffer, content_type)
        }
    }
}
