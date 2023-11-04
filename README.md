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
     <details>
     <summary>Demo</summary>
     ![root](https://github.com/Desjajja/rust_webserver/assets/58029489/3dd7890a-5aee-4441-b81c-a90cc5bc5fb1)
     </details>
   * `/echo`: echo whatever comes after it
 
     ![echo](https://github.com/Desjajja/rust_webserver/assets/58029489/77b4b549-5410-430a-bf78-9e24a08dd97b)

   * `/user-agent`: return 200 with user-agent
   * `/files`: fetch files from the assigned directory on server side. (set when running with `cargo run -- --directory <root-dir>`), invalid path get a 404.
 
     ![get_file](https://github.com/Desjajja/rust_webserver/assets/58029489/b4b6a673-85fb-4c0d-91db-cfb7c76d3690)
* `POST`

   * `/files`: create a file using body as its content, get 201 when succeeded, 500 when failed
     
![post_file](https://github.com/Desjajja/rust_webserver/assets/58029489/06f5c79d-5b63-45c8-a1a1-d457a9ee2eec)

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
