pub fn get_response(status: Status, content_type: ContentType, content: Option<String>) -> String {
	match status {
		Status::NotFound => "HTTP/1.1 404 Not Found\r\n\r\n".to_owned(),
		Status::OK => {
			if content.is_none() {
				"HTTP/1.1 200 OK\r\n\r\n".to_owned()
			} else {
				let content_type = match content_type{
					ContentType::TextPlain => "text/plain",
					_ => "application/octet-stream",
				};
				let content = content.unwrap();
				format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}", content_type, content.len(), content)
			}
		},
		Status::Created => "HTTP/1.1 201 Created\r\n\r\n".to_owned()
	}
}
pub enum Status {
	OK,
	Created,
	NotFound,
}

pub enum ContentType{
	TextPlain,
	File,
	Unknown,
	None,
}