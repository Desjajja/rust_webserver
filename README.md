# Single thread implementation
built with `tokio`, supporting methods and serving point are as follows:
* `GET`
   * `/`: return 200 with empty body
   * `/echo`: echo whatever comes after it
   * `/user-agent`: return 200 with user-agent
   * `/files`: fetch files from the assigned directory on server side. (set when running with `cargo run -- --directory <root-dir>`), invalid path get a 404.
* `POST`
   * `files`: create a file using body as its content, get 201 when succeed, 500 when failed/

## Performance Summary
method: `GET` 

serving endpoint: `http://localhost:4321/` 

concurrency: 20,000

<img width="232" alt="tokio-20000" src="https://github.com/Desjajja/rust_webserver/assets/58029489/f8070af4-a521-42e4-9900-ba374f76493c">
