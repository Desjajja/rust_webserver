# a Web Server in Rust
## Implementations
1. multithread (threadpool): on branch `master`
2. async (tokio): on branch `tokio`

## Features
1. High concurrency (as shown in section `Performance Summary`)
2. multiply methods supported(`POST`, `GET`), along with various services(echo, file IO, etc.)
3. modular architecture (as abstracted into `request` and `response` mods)

## Serving Endpoionts
* `GET`
   * `/`: return 200 with empty body
   * `/echo`: echo whatever comes after it
   * `/user-agent`: return 200 with user-agent
   * `/files`: fetch files from the assigned directory on server side. (set when running with `cargo run -- --directory <root-dir>`), invalid path get a 404.
* `POST`
   * `/files`: create a file using body as its content, get 201 when succeeded, 500 when failed


## Performance Summary
device: Laptop

Num of thread: 10

method: GET

serving endpoint: `http://localhost:4321/`

concurrency: 20,000

<img width="243" alt="thread_pool-20000" src="https://github.com/Desjajja/rust_webserver/assets/58029489/f7932ccd-942d-4965-9303-6348c29f2e32">

## Acknowledgement
[CodeCrafters.io](https://app.codecrafters.io/catalog): *this project starts from one of their their chanllenging and scrupulous challenges.*

[Rust Programming Language](https://www.rust-lang.org/): *most intelligent compiler ever!*

[oha](https://github.com/hatoo/oha): *its well designed tui and function made testing less upset*
