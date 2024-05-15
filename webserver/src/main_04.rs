use std::time::Duration;
use std::fs;
use async_std::net::TcpListener;
// use async_std::net::TcpStream; // handle_connection实际上并不需要async_std::net::TcpStream;
use async_std::io::{Read, Write}; // 它需要任何实现async_std::io::Read、async_std::io::Write和的结构marker::Unpin
use async_std::prelude::*;
use async_std::task;
use async_std::task::spawn;
use futures::StreamExt;
use webserver::{get_html_path};

#[async_std::main]
async fn main() {
    // Listen for incoming TCP connections on localhost port 7878
    let addr = String::from("0.0.0.0:7878");
    let listener = TcpListener::bind(&addr).await.unwrap(); // async version
    println!("+++ [async_web_server] listener addr:{addr}");
    
    listener.incoming().for_each_concurrent(None, |stream| async move {
        let stream = stream.unwrap();
        let remote_addr= stream.peer_addr();
        println!("+++ ==> get a stream: {:?}", remote_addr);
        
        // handle_connection(stream).await; // 并发处理
        
        spawn(handle_connection(stream)); // 单独线程执行
    }).await;
}

// async fn handle_connection(mut stream: TcpStream) {
async fn handle_connection(mut stream: impl Read + Write + Unpin) {  // 更改 的签名handle_connection以使其更易于测试, 添加impl Read + Write + Unpin 利于测试
    // Read the first 1024 bytes of data from the stream
    let mut buffer = [0; 1024];
    // stream.read(&mut buffer).unwrap();
    stream.read(&mut buffer).await.unwrap(); // async version

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // Respond with greetings or a 404,
    // depending on the data in the request
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", get_html_path(String::from("hello.html")))
    } else if buffer.starts_with(sleep) {
        println!("+++ sleep 5s Zzzz...");
        // add sleep
        task::sleep(Duration::from_secs(5)).await; // async version
        ("HTTP/1.1 200 OK\r\n\r\n", get_html_path(String::from("sleep.html")))
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", get_html_path(String::from("404.html")))
    };
    println!("+++ status_line:{status_line}, filename:{filename}");
    
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    // Write response back to the stream,
    // and flush the stream to ensure the response is sent back to the client
    let response = format!("{status_line}{contents}");

    println!("+++ <== send a response {length}");
    stream.write_all(response.as_bytes()).await.unwrap(); // async version
    stream.flush().await.unwrap(); // async version
}

#[cfg(test)]
mod async_tests {
    use super::*;
    use futures::io::Error;
    use futures::task::{Context, Poll};
    use std::cmp::min;
    use std::pin::Pin;
    
    struct MockTcpStream {
        read_data: Vec<u8>,
        write_data: Vec<u8>,
    }
    
    impl Read for MockTcpStream {
        fn poll_read(self: Pin<&mut Self>, _:&mut Context, buf: &mut [u8]) -> Poll<Result<usize, Error>> {
            let size: usize = min(self.read_data.len(), buf.len());
            buf[..size].copy_from_slice(&self.read_data[..size]);
            Poll::Ready(Ok(size))
        }
    }

    impl Write for MockTcpStream {
        fn poll_write(mut self: Pin<&mut Self>, _:&mut Context, buf: &[u8]) -> Poll<Result<usize, Error>> {
            self.write_data = Vec::from(buf);
            
            Poll::Ready(Ok(buf.len()))
        }
        
        fn poll_flush(self: Pin<&mut Self>, _:&mut Context) -> Poll<Result<(), Error>> {
            Poll::Ready(Ok(()))
        }
        
        fn poll_close(self: Pin<&mut Self>, _:&mut Context) -> Poll<Result<(), Error>>  {
            Poll::Ready(Ok(()))
        }
    }
    
    use std::marker::Unpin;
    impl Unpin for MockTcpStream {}
    
    use std::fs;
    
    #[async_std::test]
    async fn test_handle_connection() {
        let input_bytes = b"GET / HTTP/1.1\r\n";
        let mut contents = vec![0u8; 1024];
        contents[..input_bytes.len()].clone_from_slice(input_bytes);
        let mut stream = MockTcpStream {
            read_data: contents,
            write_data: Vec::new(),
        };

        handle_connection(&mut stream).await;
        let mut buf = [0u8; 1024];
        stream.read(&mut buf).await.unwrap();
        
        let expected_contents = fs::read_to_string(get_html_path(String::from("hello.html"))).unwrap();
        let expected_response = format!("HTTP/1.1 200 OK\r\n\r\n{}", expected_contents);
        assert!(stream.write_data.starts_with(expected_response.as_bytes()));
    }
    
}