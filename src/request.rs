use std::collections::HashMap;

pub struct HttpRequest<'a> { // change items to slice since HttpRequest lives within the lifetime of request
    pub method: Option<&'a str>,
    pub path: Option<&'a str>,
    pub version: Option<&'a str>,
    pub header_fields: HashMap<String, &'a str>, // key is a String since to_lowercase returns String
    pub body: Option<&'a str>,
}
pub fn parse_request (request: &str) -> Option<HttpRequest> {
    let mut method = None;
    let mut path = None;
    let mut version = None;
    let mut header_fields = HashMap::new();

    let request_split = request.split_once("\r\n\r\n")?;
    let (header, body) = request_split;

    for (idx, line) in header.lines().enumerate() {
        if idx == 0 {
            let mut metadata = line.split_whitespace();
            method = metadata.next();
            path = metadata.next();
            version = metadata.next();
        } else {
            let (k, v) = line.split_once(':')?;
            header_fields.insert(
                k.trim().to_lowercase(), //keys are stored in lowercase
                v.trim(),
            );
        }
    }
    let length = header_fields.get("content-length");
    let length:usize = match length {
        Some(length) => {
            length
                .parse() // deref coersion happens here
                .unwrap()
        },
        _ => 0,
    };

        let body = match length{
            length if length > 0 => Some(&body[..length]),
            _ => None,
        }; 
    Some(HttpRequest {header_fields, method, path, body, version})
}