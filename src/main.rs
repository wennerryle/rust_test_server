mod arguments_parser;
mod front_controller;
use std::io::{BufRead, BufReader, Read};
use std::net::{SocketAddr, TcpListener, TcpStream};

fn main() {
    let port = arguments_parser::get_port();
    let (port, listener) = get_listener(port, |port| {
        println!("Can't take port {}. Trying to take port {}", port, port + 1)
    });

    println!("\nListening on http://127.0.0.1:{}", port);

    for stream in listener.incoming() {
        let _ = stream.map(handle_connection);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let http_request = buf_reader
        .by_ref()
        .lines()
        .next()
        .map(|it| it.ok())
        .flatten();

    match http_request {
        Some(value) if value.starts_with("GET /") => {
            println!("I GOT GET! {value}");

            buf_reader
                .by_ref()
                .lines()
                .filter_map(|it| it.ok())
                .for_each(|val| println!("{val}"));
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
