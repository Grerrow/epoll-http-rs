use std::net::TcpListener;

pub fn run() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind port 8080");
    println!("Server listening on port 8080...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                // Handle the connection in a separate thread or async task
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}