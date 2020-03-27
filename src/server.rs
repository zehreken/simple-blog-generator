use std::net::TcpListener;

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:4000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
