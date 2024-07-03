use std::{
    fmt::{Result},
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use http_server_rust::ThreadPool;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").expect("could not bind");
    let pool = ThreadPool::new(100);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Lenght: {length}\r\n\r\n{contents}"); // HTTP-Version Status-Code Reason-Phrase CRLF headers(empty) CRLFm essage-body(empty)
    stream.write_all(response.as_bytes()).unwrap();
}
