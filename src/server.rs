use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

pub fn start_server() {
    // Open browser, this works but running this in another thread after making sure
    // that serve is started is a better idea, this only works on MacOs
    std::process::Command::new("open")
        .arg(String::from("http://127.0.0.1:4000"))
        .output();

    let listener = TcpListener::bind("127.0.0.1:4000").unwrap();
    let pool = crate::lib::ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down")
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        println!("{:?}", &buffer[..]);
        ("HTTP/1.1 200 OK\r\n\r\n", "site/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "site/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
