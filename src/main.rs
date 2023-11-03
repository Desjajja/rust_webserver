use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::fs::File;
use http_server_starter_rust::response::*;
use http_server_starter_rust::request::{HttpRequest, parse_request};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut args = std::env::args();
    let mut root_dir = None;
    if let Some(arg) = args.nth(1) {
        if arg == "--directory" {
            root_dir = args.last();
        }
    }

    let listener = TcpListener::bind("127.0.0.1:4221").await.unwrap();
    while let Ok((mut socket, _)) = listener.accept().await {
        let root_dir = root_dir.clone(); // TODO: change this to shared memo or something...
        tokio::spawn(async move {
            if let Err(err) = process_request(&mut socket, root_dir).await {
                eprintln!("Error processing request: {:?}", err);
            }
        });
    }
    Ok(())
}

async fn process_request(socket: &mut TcpStream, root_dir: Option<String>) -> Result<(), std::io::Error> {
    let mut buffer = [0u8; 2048];
    socket.read(&mut buffer).await?;

    let request = String::from_utf8_lossy(& buffer);
    let request = parse_request(&request).unwrap();
    let method = request.method.unwrap().to_lowercase();
    let uri = request.path.unwrap();
    let response = match method {
        method if method == "post" => {
            match uri {
                uri if uri.starts_with("/files") => post_file(request, root_dir).await,
                _ => respond_405(),
            }
        },
        method if method =="get" => {
            match uri {
                uri if uri == "/" => respond_200(),
                uri if uri.starts_with("/echo") => handle_echo(request),
                uri if uri.starts_with("/user-agent") => handle_user_agent(request),
                uri if uri.starts_with("/files") => get_files(request, root_dir).await,
                _ => respond_404(),
            }
        },
        _ => respond_404(),
    };
    socket.write_all(response.as_bytes()).await?;
    Ok(())
}

async fn post_file(req: HttpRequest<'_>, root_dir: Option<String>) -> String {
    let filename = req.path.unwrap().split('/').last().unwrap().to_owned();
    let mut path_buffer = PathBuf::new();
    path_buffer.push(PathBuf::from(root_dir.unwrap_or_default()));
    path_buffer.push(PathBuf::from(filename));
    match File::create(path_buffer).await {
        Ok(mut f) => f.write_all(req.body.unwrap().as_bytes()).await.unwrap(),
        Err(_) => return respond_404(),
    }
    respond_201()
}

fn handle_echo(req: HttpRequest) -> String {
    let content = req.path.unwrap().split_once("/echo/").unwrap().1.to_owned();
    respond_200_with_text(content)
}


fn handle_user_agent(req: HttpRequest) -> String{
    let content = req.header_fields.get("user-agent").unwrap().to_owned().to_owned();
    respond_200_with_text(content)
}

async fn get_files(req: HttpRequest<'_>, root_dir: Option<String>) -> String {
    let filename = req.path.unwrap().split('/').last().unwrap().to_owned();
    let mut path_buffer = PathBuf::new();
    path_buffer.push(PathBuf::from(root_dir.unwrap_or_default()));
    path_buffer.push(PathBuf::from(filename));
    if !path_buffer.exists() {
        return respond_404();
    }

    let mut buffer = String::new();
    let mut f =  File::open(path_buffer).await.unwrap();
    f.read_to_string(&mut buffer).await.unwrap();
    respond_200_with_file(buffer)
}
