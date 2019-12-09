mod messages;
mod rbgp;

use chrono::Utc;
use std::time::Duration;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    println!("Starting server...");

    let addr = "127.0.0.1:6142";
    let mut listener = TcpListener::bind(addr).await.unwrap();

    let server = async move {
        // Wait for connections. On receiving a connection spawn it off into a new "green thread"
        // and run the handler.
        rbgp::run_loop::run_loop(listener)
    };

    println!("Server is running on {0}", addr);
    server.await;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
