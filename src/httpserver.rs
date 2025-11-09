use std::collections::VecDeque;
use std::{fs::File, str::FromStr};
use tiny_http::{Header, Method, Response, Server};

use crate::utils::{read_dir, resolve_path};

pub struct HttpServer {
    port: u16,
    ip: String,
}

impl HttpServer {
    pub fn new(ip: &str, port: u16) -> HttpServer {
        HttpServer {
            ip: ip.to_string(),
            port: port,
        }
    }
    pub fn run(&self) {
        let server = Server::http(format!("{}:{}", self.ip, self.port)).unwrap();
        for request in server.incoming_requests() {
            if request.method() != &Method::Get {
                let response = Response::from_string("Only get method").with_status_code(403);
                request.respond(response).unwrap();
                continue;
            }

            if let Some(name) = request.url().to_string().strip_prefix("/") {
                let name = name.trim_end_matches("/");
                let path = resolve_path(name).expect("Path error");
                
                // Generate HTML code for the path where the program is running
                if path.is_dir() {
                    let mut response = r#"<!DOCTYPE HTML>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Directory listing</title>
</head>
<body>
<h1>Directory listing</h1><hr><ul>"#
                        .to_string();
                    if let Some(filenames) = read_dir(&path).ok() {
                        for filename in filenames {
                            let mut filepath = path.clone();
                            filepath.push(&filename);
                            if filepath.is_dir() {
                                let mut current_dir_user: VecDeque<&str> =
                                    name.split("/").collect();
                                while current_dir_user.len() > 1 {
                                    current_dir_user.pop_front();
                                }
                                let current_dir_user: Vec<&str> =
                                    current_dir_user.into_iter().collect();
                                let current_dir_user = current_dir_user.join("/");
                                response.push_str(
                                    format!(
                                        "<li><a href=\"{}\">{}/</a></li>\n",
                                        format!("{}/{}", current_dir_user, filename),
                                        filename
                                    )
                                    .as_str(),
                                );
                            } else if filepath.is_file() {
                                let mut current_dir_user: VecDeque<&str> =
                                    name.split("/").collect();
                                while current_dir_user.len() > 1 {
                                    current_dir_user.pop_front();
                                }
                                let current_dir_user: Vec<&str> =
                                    current_dir_user.into_iter().collect();
                                let current_dir_user = current_dir_user.join("/");
                                response.push_str(
                                    format!(
                                        "<li><a href=\"{}\">{}</a></li>\n",
                                        format!("{}/{}", current_dir_user, filename),
                                        filename
                                    )
                                    .as_str(),
                                );
                            }
                        }
                        response.push_str("</ul><hr></body></html>");
                    }
                    let _ = request.respond(
                        Response::from_string(response)
                            .with_header(Header::from_str("Content-Type: text/html").unwrap()),
                    );
                } else if path.is_file() {
                    if let Ok(file) = File::open(path.clone()) {
                        if request
                            .respond(
                                Response::from_file(file)
                                    .with_header(
                                        Header::from_str("Content-Type: application/octet-stream")
                                            .unwrap(),
                                    )
                                    .with_header(
                                        Header::from_str(
                                            format!(
                                                "Content-Disposition: attachment; filename=\"{}\"",
                                                name
                                            )
                                            .as_str(),
                                        )
                                        .unwrap(),
                                    ),
                            )
                            .is_ok()
                        {
                            continue;
                        }
                    }
                }
            }
        }
    }
}
