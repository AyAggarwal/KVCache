// Import the server module
use kvcache::net::Server;

fn main() {
    // Create a new server on localhost port 3000
    let server = Server::new();
    
    println!("Starting TCP server...");
    
    // Start the server and handle any errors
    if let Err(e) = server.start() {
        eprintln!("Failed to start the server: {}", e);
    }
}