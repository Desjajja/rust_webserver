use std::collections::HashMap;

pub struct HttpRequest {
    pub method: Option<String>,
    pub path: Option<String>,
    pub version: Option<String>,
    pub header_fields: HashMap<String, String>,
    pub body: Option<String>,
}
pub fn parse_request<'a> (request: &'a str) -> Option<HttpRequest> {
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