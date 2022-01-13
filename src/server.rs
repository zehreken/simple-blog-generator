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
            "Received request\nMethod: {:?}\nPath: {:?}\n",
            request.method(),
            request.url(),
        );

        match fs::read_dir(format!("site{}", request.url())) {
            Ok(_) => {
                if let Ok(file) = fs::File::open(format!("site{}/index.html", request.url())) {
                    let response = Response::from_file(file);
                    request.respond(response).unwrap();
                }
            }
            Err(_) => {
                if let Ok(file) = fs::File::open(format!("site{}", request.url())) {
                    let response = Response::from_file(file);
                    request.respond(response).unwrap();
                }
            }
        }
    }
}
