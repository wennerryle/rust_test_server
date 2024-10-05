mod arguments_parser;
mod front_controller;
mod request_parser;
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

fn main() {
    let port = arguments_parser::get_port();
    let (port, listener) = get_listener(port, |port| {
        println!("Can't take port {}. Trying to take port {}", port, port + 1)
    });

    if port < 1023 {
        println!("notice: nonadministrators can listen only on ports higher than 1023");
    }

    println!("\nListening on http://127.0.0.1:{port}\n");

    for stream in listener.incoming() {
        let _ = stream.map(handle_connection);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let mut http_request = buf_reader
        .lines()
        .filter_map(Result::ok)
        .take_while(|line| !line.is_empty())
        .into_iter();

    let fst_line = http_request.next();

    match fst_line {
        Some(value) if value.starts_with("GET /") => {
            let status_line = "HTTP/1.1 200 OK";
            let contents = fs::read_to_string(r"index.html").unwrap();
            let length = contents.len();

            println!("Headers:");
            http_request.for_each(|header| println!("{header}"));

            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

            let _ = stream.write_all(response.as_bytes());
        }
        _ => {}
    }
}

fn get_listener(port: u16, on_fault: fn(u16)) -> (u16, TcpListener) {
    let mut current_port = port;
    let get_addr = |port| SocketAddr::from(([127, 0, 0, 1], port));
    let mut listener = TcpListener::bind(get_addr(port));

    while listener.is_err() {
        on_fault(current_port);
        current_port = current_port.checked_add(1).unwrap_or(0);
        listener = TcpListener::bind(get_addr(current_port));
    }

    (current_port, listener.unwrap())
}
