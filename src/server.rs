use std::fs;
use tiny_http::{Response, Server};

pub fn start(address: &str) {
    // Open browser, this works but running this in another thread after making sure
    // that server is started is a better idea, this only works on macOS
    std::process::Command::new("open")
        .arg(String::from(address))
        .output()
        .unwrap();

    let server = Server::http("127.0.0.1:4000").unwrap();

    for request in server.incoming_requests() {
        println!(
            "received request! method: {:?}\n, url: {:?}\n, headers: {:?}\n",
            request.method(),
            request.url(),
            request.headers()
        );

        let file = fs::File::open(format!("site{}", request.url()));
        match file {
            Ok(_) => {
                let response = Response::from_file(file.unwrap());
                request.respond(response).unwrap();
            }
            Err(error) => println!("I/O ERROR: {}", error),
        }
    }
}
