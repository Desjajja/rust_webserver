pub fn respond_201() -> String {
    get_response(Status::Created, ContentType::None, None)
}

pub fn respond_200() -> String {
    get_response(Status::OK, ContentType::Unknown, None)
}

pub fn respond_200_with_text(content: String) -> String {
	get_response(Status::OK, ContentType::TextPlain, Some(content))
}

pub fn respond_200_with_file(content: String) -> String {
	get_response(Status::OK, ContentType::File, Some(content))
}



pub fn respond_404() -> String {
    get_response(Status::NotFound, ContentType::Unknown, None)
}

pub fn respond_405() -> String {
	get_response(Status::NotAllowed, ContentType::None , None)
}

pub fn respond_500() -> String {
	get_response(Status::InternalServerError, ContentType::None, None)
}

fn get_response(status: Status, content_type: ContentType, content: Option<String>) -> String {
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
		Status::Created => "HTTP/1.1 201 Created\r\n\r\n".to_owned(),
		Status::NotAllowed => "HTTP/1.1 405 Method Not Allowed\r\n\r\n".to_owned(),
		Status::InternalServerError => "HTTP/1.1 500 Internal Server Error\r\n\r\n".to_owned(),
	}
}
enum Status {
	OK,
	Created,
	NotFound,
	NotAllowed,
	InternalServerError,
}

enum ContentType{
	TextPlain,
	File,
	Unknown,
	None,
}