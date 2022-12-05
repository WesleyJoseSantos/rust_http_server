use std::{
    str,
    fs,
    io::{prelude::*},
    net::{TcpListener, TcpStream}, time::Duration,
};

const SERVER_URL: &'static str = "192.168.50.215:7878";
const INDEX_FILENAME: &'static str = "index.html";
const STATUS_LED_FILENAME: &'static str = "status_led.txt";

fn main() {
    let listener = TcpListener::bind(SERVER_URL).unwrap();

    println!("Application is running at http://{}", SERVER_URL);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut data = String::new();
    stream.set_read_timeout(Some(Duration::from_millis(150))).unwrap();
    stream.read_to_string(&mut data).unwrap_or(0);
    let mut lines = data.lines();

    let request_line = lines.next().unwrap_or("");

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string(INDEX_FILENAME).unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    } else if request_line == "GET /status_led HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string(STATUS_LED_FILENAME).unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    } else if request_line == "POST /status_led HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = String::from("OK");
        let length = contents.len();
        let mut body : &str = "teste";

        for line in lines {
            body = line;
        }

        fs::write(STATUS_LED_FILENAME, body).unwrap();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}
