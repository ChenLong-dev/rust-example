use webserver::{get_html_path, grace};
use std::{fs, io};
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() ->io::Result<()> {
    let addr = String::from("0.0.0.0:7878");
    let listener = TcpListener::bind(&addr).unwrap();
    let pool = grace::ThreadPool::new(4);
    println!("+++ [grace_drop_web_server] listener addr:{addr}");

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        let remote_addr= stream.peer_addr()?;
        println!("+++ ==> get a stream: {}", remote_addr);
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("--- Shutting down.");
    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", get_html_path(String::from("hello.html")))
    } else if buffer.starts_with(sleep) {
        println!("+++ sleep 5s Zzzz...");
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", get_html_path(String::from("sleep.html")))
    } else {
        ("HTTP/1.1 404 NOT FOUND", get_html_path(String::from("404.html")))
    };
    println!("+++ status_line:{status_line}, filename:{filename}");

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        length,
        contents
    );

    println!("+++ <== send a response {length}");
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}