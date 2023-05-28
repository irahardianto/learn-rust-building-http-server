use super::server::Handler;
use super::http::{Request, Response, StatusCode, Method};
use std::fs;

pub struct WebsiteHandler{
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self {
            public_path,
        }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        
        fs::canonicalize(path).ok()
            .and_then(|full_path| {
                if full_path.starts_with(&self.public_path) {
                    fs::read_to_string(full_path).ok()
                } else {
                    println!("Directory traversal attack attempted: {}", file_path);
                    None
                }
            })
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &crate::http::Request) -> crate::http::Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(
                    StatusCode::Ok,
                    self.read_file("index.html"),
                ),
                "/hello" => Response::new(
                    StatusCode::Ok,
                    self.read_file("hello.html"),
                ),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(
                        StatusCode::Ok,
                        Some(contents),
                    ),
                    None => Response::new(
                        StatusCode::NotFound,
                        Some("<h1>404 Not Found</h1>".to_string()),
                    ),
                }
            },
            _ => Response::new(
                StatusCode::NotFound,
                Some("<h1>404 Not Found</h1>".to_string()),
            ),
        }
    }
}
