use rustserver::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move || {});
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let mut response = String::new();

    if buffer.starts_with(get) {
        response = build_response(200, None);
    } else if buffer.starts_with(b"GET /hello-world HTTP/1.1\r\n") {
        response = build_response(200, Some("hello-world".to_string()));
    } else {
        response = build_response(404, None);
    }
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn build_response(status_code: i16, path: Option<String>) -> String {
    let contents: (String, String);
    println!("{}", status_code);
    if status_code == 200 {
        let file_contents_as_string = match path {
            Some(filename) => {
                let full_path = [filename, ".html".to_string()].join("");
                println!("{}", full_path);
                read_file(full_path)
            }
            None => read_file(String::from("index.html")),
        };
        contents = ("Ok".to_string(), file_contents_as_string);
        println!("{}", contents.1);
    } else {
        contents = (
            "NOT FOUND".to_string(),
            read_file("not-found.html".to_string()),
        );
    }

    format!(
        "HTTP/1.1 {} {}\r\n\r\n{}",
        status_code, contents.0, contents.1
    )
}

fn read_file(path: String) -> String {
    fs::read_to_string(path).unwrap()
}
