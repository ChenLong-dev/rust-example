use std::{
    fs,
    io::{self, prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use webserver::get_html_path;

fn main() ->io::Result<()> {
    let addr = String::from("0.0.0.0:7878");
    let listener = TcpListener::bind(&addr).unwrap();
    println!("+++ [single_threaded_web_server] listener addr:{addr}");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let remote_addr= stream.peer_addr()?;
        println!("+++ ==> get a stream: {}", remote_addr);
        handle_connection(stream);
    }
    Ok(())
}
// --snip--

fn handle_connection(mut stream: TcpStream) {
    // --snip--
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", get_html_path(String::from("hello.html")))
    } else {
        ("HTTP/1.1 404 NOT FOUND", get_html_path(String::from("404.html")))
    };
    println!("+++ status_line:{status_line}, filename:{filename}");

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    println!("+++ <== send a response {length}");
    stream.write_all(response.as_bytes()).unwrap();
}




