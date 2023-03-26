use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    //  listen for TCP connections at the address 127.0.0.1:7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // Listening for incoming streams and printing a message when we receive a stream.
    for stream in listener.incoming() {
        let stream = stream.unwrap(); // streams of type TcpStream

        // println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    // BufReader implements the std::io::BufRead trait, which provides the lines method. 
    // The lines method returns an iterator of Result<String, std::io::Error>
    
    let (status_line, contents) = if http_request[0] == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK",  String::from("<!DOCTYPE html><head><title>Hello!</title></head>Hi from Rust</html>"))
    } else {
        ("HTTP/1.1 404 NOT FOUND", String::from("<!DOCTYPE html><head><title>Hello!</title></head><h1>Oops!</h1>
        <p>Sorry, I don't know what you're asking for.</p></html>"))
    };
    let length = contents.len();
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}