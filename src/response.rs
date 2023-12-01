pub fn respond_201() -> Vec<u8> {
    get_response(Status::Created, ContentType::None, None)
}

pub fn respond_200() -> Vec<u8> {
    get_response(Status::OK, ContentType::Unknown, None)
}

pub fn respond_200_with_content(content: Vec<u8>, content_type: ContentType) -> Vec<u8> {
	get_response(Status::OK, content_type, Some(content))
}

pub fn respond_200_with_text(content: Vec<u8>) -> Vec<u8> {
	get_response(Status::OK, ContentType::TextPlain, Some(content))
}

pub fn respond_200_with_file(content: Vec<u8>, content_type: ContentType) -> Vec<u8> {
	get_response(Status::OK, content_type, Some(content))
}



pub fn respond_404() -> Vec<u8> {
    get_response(Status::NotFound, ContentType::Unknown, None)
}

pub fn respond_405() -> Vec<u8> {
	get_response(Status::NotAllowed, ContentType::None , None)
}

pub fn respond_500() -> Vec<u8> {
	get_response(Status::InternalServerError, ContentType::None, None)
}

fn get_response(status: Status, content_type: ContentType, content: Option<Vec<u8>>) -> Vec<u8> {
	match status {
		Status::NotFound => "HTTP/1.1 404 Not Found\r\n\r\n".as_bytes().to_owned(),
		Status::OK => {
			if content.is_none() {
				"HTTP/1.1 200 OK\r\n\r\n".as_bytes().to_owned()
			} else {
				let content_type = match content_type{
					ContentType::TextPlain => "text/plain",
					ContentType::Image => "image/jpeg",
					_ => "application/octet-stream",
				};
				let content = content.unwrap();
				let mut rst = format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n", content_type, content.len()).as_bytes().to_owned();
				rst.extend(content.iter());
				rst
			}
		},
		Status::Created => "HTTP/1.1 201 Created\r\n\r\n".as_bytes().to_owned(),
		Status::NotAllowed => "HTTP/1.1 405 Method Not Allowed\r\n\r\n".as_bytes().to_owned(),
		Status::InternalServerError => "HTTP/1.1 500 Internal Server Error\r\n\r\n".as_bytes().to_owned(),
	}
}
enum Status {
	OK,
	Created,
	NotFound,
	NotAllowed,
	InternalServerError,
}

pub enum ContentType{
	TextPlain,
	Textfile,
	Image,
	Unknown,
	None,
}