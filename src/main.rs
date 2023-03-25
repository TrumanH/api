use std::{
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
    
    let response = "HTTP/1.1 200 OK\r\n\r\n"; // empty body
    stream.write_all(response.as_bytes()).unwrap();
}