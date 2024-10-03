use std::env;

pub fn get_port() -> u16 {
    env::args()
        .collect::<Vec<String>>()
        .windows(2)
        .skip(1)
        .next()
        .filter(|args| args[0] == "-p")
        .and_then(|args| args[1].parse::<u16>().ok())
        .unwrap_or(7878)
}
