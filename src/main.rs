mod messages;
mod rbgp;

use chrono::Utc;
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    if false {
        println!("Starting server...");

        let addr = "127.0.0.1:6142";
        let mut listener = TcpListener::bind(addr).await.unwrap();

        let server = async move {
            // Wait for connections. On receiving a connection spawn it off into a new "green thread"
            // and run the handler.
            rbgp::run_loop::run_loop(listener).await;
        };

        println!("Server is running on {0}", addr);
        server.await;
        println!("Server is shutting down");
    } else {
        let addr = "localhost:2179";

        println!("Making connection to: {}", addr);
        let connection = rbgp::connection::Connection::new(addr).await.unwrap();
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
